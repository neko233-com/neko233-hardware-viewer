#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod hardware;
mod scoring;

use hardware::HardwareContext;
use serde::Serialize;
use std::process::Command;
use std::sync::Mutex;
use sysinfo::System;
use winreg::enums::*;
use winreg::RegKey;

struct AppState {
    sys: Mutex<System>,
}

#[derive(Serialize)]
struct SystemUsage {
    cpu_usage: f32,
    memory_used: u64,
    memory_total: u64,
}

#[tauri::command]
fn get_system_usage(state: tauri::State<AppState>) -> SystemUsage {
    let mut sys = state.sys.lock().unwrap();
    sys.refresh_cpu_all();
    sys.refresh_memory();
    
    let cpu_usage = sys.global_cpu_usage();
    let memory_used = sys.used_memory();
    let memory_total = sys.total_memory();

    SystemUsage {
        cpu_usage,
        memory_used,
        memory_total,
    }
}

// Define structs for the frontend response
#[derive(Serialize)]
struct ScoredCpu {
    info: hardware::cpu::CpuInfo,
    score: String,
    score_num: u32,
}

#[derive(Serialize)]
struct ScoredGpu {
    info: hardware::gpu::GpuInfo,
    score: String,
    score_num: u32,
}

#[derive(Serialize)]
struct ScoredRam {
    info: Vec<hardware::memory::MemoryInfo>,
    total_gb: u64,
    avg_speed: u32,
    score: String,
    score_num: u32,
}

#[derive(Serialize)]
struct ScoredDisk {
    info: hardware::disk::DiskInfo,
    score: String,
    score_num: u32,
}

#[derive(Serialize)]
struct FullHardwareInfo {
    motherboard: Vec<hardware::motherboard::MotherboardInfo>,
    cpu: Vec<ScoredCpu>,
    gpu: Vec<ScoredGpu>,
    ram: ScoredRam,
    disks: Vec<ScoredDisk>,
    sound: Vec<hardware::sound::SoundInfo>,
}

#[tauri::command]
fn get_hardware_info() -> Result<FullHardwareInfo, String> {
    let ctx = HardwareContext::new().map_err(|e| e.to_string())?;

    let motherboard = hardware::motherboard::get_motherboard_info(&ctx).map_err(|e| e.to_string())?;
    
    let cpus = hardware::cpu::get_cpu_info(&ctx).map_err(|e| e.to_string())?;
    let scored_cpus = cpus.into_iter().map(|cpu| {
        let score = scoring::score_cpu(cpu.number_of_cores, cpu.max_clock_speed);
        let score_num = scoring::calculate_cpu_score_num(cpu.number_of_cores, cpu.max_clock_speed);
        ScoredCpu {
            info: cpu,
            score: format!("{:?}", score),
            score_num,
        }
    }).collect();

    let gpus = hardware::gpu::get_gpu_info(&ctx).map_err(|e| e.to_string())?;
    let scored_gpus = gpus.into_iter().map(|gpu| {
        let (score, score_num) = if let Some(ram) = gpu.adapter_ram {
             (format!("{:?}", scoring::score_gpu(ram)), scoring::calculate_gpu_score_num(ram))
        } else {
             ("Unknown".to_string(), 0)
        };
        ScoredGpu {
            info: gpu,
            score,
            score_num,
        }
    }).collect();

    let mems = hardware::memory::get_memory_info(&ctx).map_err(|e| e.to_string())?;
    let mut total_cap = 0;
    let mut speeds = Vec::new();
    for mem in &mems {
        total_cap += mem.capacity;
        let speed = mem.configured_clock_speed.unwrap_or(mem.speed);
        speeds.push(if speed > 0 { speed } else { mem.speed });
    }
    let total_gb = total_cap / 1024 / 1024 / 1024;
    let avg_speed = if !speeds.is_empty() {
        speeds.iter().sum::<u32>() / speeds.len() as u32
    } else {
        0
    };
    let ram_score = format!("{:?}", scoring::score_ram(total_gb, avg_speed));
    let ram_score_num = scoring::calculate_ram_score_num(total_gb, avg_speed);
    let scored_ram = ScoredRam {
        info: mems,
        total_gb,
        avg_speed,
        score: ram_score,
        score_num: ram_score_num,
    };

    let disks = hardware::disk::get_disk_info(&ctx).map_err(|e| e.to_string())?;
    let scored_disks = disks.into_iter().map(|disk| {
        let (score, score_num) = if let Some(size) = disk.size {
            (format!("{:?}", scoring::score_disk(size)), scoring::calculate_disk_score_num(size))
        } else {
            ("Unknown".to_string(), 0)
        };
        ScoredDisk {
            info: disk,
            score,
            score_num,
        }
    }).collect();

    let sound = hardware::sound::get_sound_info(&ctx).map_err(|e| e.to_string())?;

    Ok(FullHardwareInfo {
        motherboard,
        cpu: scored_cpus,
        gpu: scored_gpus,
        ram: scored_ram,
        disks: scored_disks,
        sound,
    })
}

#[tauri::command]
fn get_firewall_status() -> bool {
    // Simple check using netsh
    let output = Command::new("netsh")
        .args(&["advfirewall", "show", "allprofiles", "state"])
        .output();
    
    if let Ok(out) = output {
        let stdout = String::from_utf8_lossy(&out.stdout);
        // If any profile is ON, we consider it ON for simplicity, or check if ALL are ON.
        // Output format: "State ON" or "State OFF"
        return stdout.to_lowercase().contains("on");
    }
    false
}

#[tauri::command]
fn set_firewall_status(enable: bool) -> Result<String, String> {
    let state = if enable { "on" } else { "off" };
    let output = Command::new("netsh")
        .args(&["advfirewall", "set", "allprofiles", "state", state])
        .output()
        .map_err(|e| e.to_string())?;
    
    if output.status.success() {
        Ok(format!("Firewall turned {}", state))
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

#[tauri::command]
fn get_cortana_status() -> bool {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    if let Ok(key) = hklm.open_subkey("SOFTWARE\\Policies\\Microsoft\\Windows\\Windows Search") {
        if let Ok(val) = key.get_value::<u32, _>("AllowCortana") {
            return val == 1;
        }
    }
    // Default is enabled if not set
    true
}

#[tauri::command]
fn set_cortana_status(enable: bool) -> Result<String, String> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let (key, _) = hklm.create_subkey("SOFTWARE\\Policies\\Microsoft\\Windows\\Windows Search")
        .map_err(|e| e.to_string())?;
    
    let val: u32 = if enable { 1 } else { 0 };
    key.set_value("AllowCortana", &val).map_err(|e| e.to_string())?;
    
    Ok(format!("Cortana {}", if enable { "enabled" } else { "disabled" }))
}

#[tauri::command]
fn get_activation_status() -> String {
    let output = Command::new("cscript")
        .args(&["//Nologo", "C:\\Windows\\System32\\slmgr.vbs", "/xpr"])
        .output();
        
    if let Ok(out) = output {
        return String::from_utf8_lossy(&out.stdout).trim().to_string();
    }
    "Unknown".to_string()
}

#[derive(Serialize, Clone)]
struct DriverInfo {
    published_name: String,
    original_name: String,
    provider_name: String,
    class_name: String,
    version: String,
    date: String,
}

#[tauri::command]
fn scan_graphic_drivers() -> Result<Vec<DriverInfo>, String> {
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x08000000;

    let output = Command::new("pnputil")
        .args(&["/enum-drivers", "/class", "Display"])
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .map_err(|e| e.to_string())?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut drivers = Vec::new();
    
    let mut current_driver = DriverInfo {
        published_name: String::new(),
        original_name: String::new(),
        provider_name: String::new(),
        class_name: String::new(),
        version: String::new(),
        date: String::new(),
    };
    
    for line in stdout.lines() {
        let line = line.trim();
        if line.starts_with("Published Name:") {
            if !current_driver.published_name.is_empty() {
                drivers.push(current_driver.clone());
            }
            current_driver = DriverInfo {
                published_name: line.replace("Published Name:", "").trim().to_string(),
                original_name: String::new(),
                provider_name: String::new(),
                class_name: String::new(),
                version: String::new(),
                date: String::new(),
            };
        } else if line.starts_with("Original Name:") {
            current_driver.original_name = line.replace("Original Name:", "").trim().to_string();
        } else if line.starts_with("Provider Name:") {
            current_driver.provider_name = line.replace("Provider Name:", "").trim().to_string();
        } else if line.starts_with("Class Name:") {
            current_driver.class_name = line.replace("Class Name:", "").trim().to_string();
        } else if line.starts_with("Driver Version:") {
            current_driver.version = line.replace("Driver Version:", "").trim().to_string();
        } else if line.starts_with("Driver Date:") {
            current_driver.date = line.replace("Driver Date:", "").trim().to_string();
        }
    }
    if !current_driver.published_name.is_empty() {
        drivers.push(current_driver);
    }

    // Filter for NVIDIA or AMD
    let filtered: Vec<DriverInfo> = drivers.into_iter().filter(|d| {
        let p = d.provider_name.to_lowercase();
        p.contains("nvidia") || p.contains("amd") || p.contains("advanced micro devices") || p.contains("intel")
    }).collect();

    Ok(filtered)
}

#[tauri::command]
fn uninstall_driver(published_name: String) -> Result<String, String> {
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x08000000;

    let output = Command::new("pnputil")
        .args(&["/delete-driver", &published_name, "/uninstall", "/force"])
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .map_err(|e| e.to_string())?;
        
    if output.status.success() {
        Ok("Success".to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

#[tauri::command]
fn install_product_key(key: String) -> Result<String, String> {
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x08000000;

    let output = Command::new("cscript")
        .args(&["//Nologo", "C:\\Windows\\System32\\slmgr.vbs", "/ipk", &key])
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok("Success".to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

#[tauri::command]
fn attempt_activation() -> Result<String, String> {
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x08000000;

    let output = Command::new("cscript")
        .args(&["//Nologo", "C:\\Windows\\System32\\slmgr.vbs", "/ato"])
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok("Success".to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

#[tauri::command]
fn set_win11_bypass() -> Result<String, String> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let setup = hklm.open_subkey("SYSTEM\\Setup").map_err(|e| e.to_string())?;
    let (lab_config, _) = setup.create_subkey("LabConfig").map_err(|e| e.to_string())?;

    lab_config.set_value("BypassTPMCheck", &1u32).map_err(|e| e.to_string())?;
    lab_config.set_value("BypassSecureBootCheck", &1u32).map_err(|e| e.to_string())?;
    lab_config.set_value("BypassRAMCheck", &1u32).map_err(|e| e.to_string())?;
    lab_config.set_value("BypassStorageCheck", &1u32).map_err(|e| e.to_string())?;
    lab_config.set_value("BypassCPUCheck", &1u32).map_err(|e| e.to_string())?;

    Ok("Success".to_string())
}

#[tauri::command]
fn set_classic_context_menu(enable: bool) -> Result<String, String> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let key_path = "Software\\Classes\\CLSID\\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}\\InprocServer32";
    
    if enable {
        let (key, _) = hkcu.create_subkey(key_path).map_err(|e| e.to_string())?;
        key.set_value("", &"").map_err(|e| e.to_string())?;
    } else {
        // To restore default, we delete the key
        // We need to delete the parent CLSID key if it's empty, but for now just deleting InprocServer32 is enough or the CLSID itself
        let _ = hkcu.delete_subkey_all("Software\\Classes\\CLSID\\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}");
    }
    Ok("Success".to_string())
}

#[tauri::command]
fn set_show_extensions(enable: bool) -> Result<String, String> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let advanced = hkcu.open_subkey_with_flags("Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced", KEY_WRITE).map_err(|e| e.to_string())?;
    
    // HideFileExt: 0 = Show, 1 = Hide
    let val = if enable { 0u32 } else { 1u32 };
    advanced.set_value("HideFileExt", &val).map_err(|e| e.to_string())?;
    
    Ok("Success".to_string())
}

#[tauri::command]
fn set_show_hidden_files(enable: bool) -> Result<String, String> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let advanced = hkcu.open_subkey_with_flags("Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced", KEY_WRITE).map_err(|e| e.to_string())?;
    
    // Hidden: 1 = Show, 2 = Hide
    let val = if enable { 1u32 } else { 2u32 };
    advanced.set_value("Hidden", &val).map_err(|e| e.to_string())?;
    
    Ok("Success".to_string())
}

#[tauri::command]
fn restart_explorer() -> Result<String, String> {
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x08000000;

    // Kill explorer
    let _ = Command::new("taskkill")
        .args(&["/F", "/IM", "explorer.exe"])
        .creation_flags(CREATE_NO_WINDOW)
        .output();

    // Start explorer
    Command::new("explorer.exe")
        .creation_flags(CREATE_NO_WINDOW)
        .spawn()
        .map_err(|e| e.to_string())?;

    Ok("Success".to_string())
}

#[tauri::command]
fn run_memory_diagnostic() -> Result<String, String> {
    // mdsched.exe is the Windows Memory Diagnostic tool
    // It usually requires admin privileges and shows a GUI
    Command::new("mdsched.exe")
        .spawn()
        .map_err(|e| e.to_string())?;

    Ok("Success".to_string())
}

fn main() {
    let sys = System::new_all();
    let app_state = AppState {
        sys: Mutex::new(sys),
    };

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            get_hardware_info, 
            get_system_usage,
            get_firewall_status,
            set_firewall_status,
            get_cortana_status,
            set_cortana_status,
            get_activation_status,
            scan_graphic_drivers,
            uninstall_driver,
            install_product_key,
            attempt_activation,
            set_win11_bypass,
            set_classic_context_menu,
            set_show_extensions,
            set_show_hidden_files,
            restart_explorer,
            run_memory_diagnostic
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
