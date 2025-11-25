#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod hardware;
mod scoring;

use hardware::HardwareContext;
use serde::{Deserialize, Serialize};
use std::process::Command;
use std::sync::Mutex;
use std::fs;
use sysinfo::System;
use tauri::Emitter;
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

#[derive(Serialize, Deserialize)]
struct AppConfig {
    tab_order: Vec<String>,
}

#[tauri::command]
fn save_tab_order(order: Vec<String>) -> Result<(), String> {
    let config = AppConfig { tab_order: order };
    let json = serde_json::to_string_pretty(&config).map_err(|e| e.to_string())?;
    fs::write("neko233-hardware-viewer.config.json", json).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn load_tab_order() -> Result<Vec<String>, String> {
    if let Ok(content) = fs::read_to_string("neko233-hardware-viewer.config.json") {
        if let Ok(config) = serde_json::from_str::<AppConfig>(&content) {
            return Ok(config.tab_order);
        }
    }
    Ok(vec![])
}

#[tauri::command]
fn get_boot_time() -> u64 {
    System::boot_time()
}

#[tauri::command]
fn get_system_usage(state: tauri::State<AppState>) -> SystemUsage {
    let mut sys = state.sys.lock().unwrap();
    sys.refresh_all();
    
    let cpu_usage = sys.global_cpu_info().cpu_usage();
    let memory_used = sys.used_memory();
    let memory_total = sys.total_memory();

    SystemUsage {
        cpu_usage,
        memory_used,
        memory_total,
    }
}

// 定义前端响应的结构体
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
    monitor: Vec<hardware::monitor::MonitorInfo>,
    network: Vec<hardware::network::NetworkInfo>,
    usb: Vec<hardware::peripherals::PnPDevice>,
    camera: Vec<hardware::peripherals::PnPDevice>,
    bluetooth: Vec<hardware::peripherals::PnPDevice>,
}

#[tauri::command]
fn get_hardware_info() -> Result<FullHardwareInfo, String> {
    // Parallelize hardware scans to improve startup time
    
    let motherboard_handle = std::thread::spawn(|| {
        let mut ctx = HardwareContext::new();
        ctx.init_wmi().map_err(|e| e.to_string())?;
        hardware::motherboard::get_motherboard_info(&ctx).map_err(|e| e.to_string())
    });

    let cpu_handle = std::thread::spawn(|| {
        let mut ctx = HardwareContext::new();
        // CPU uses sysinfo primarily, so we don't init WMI unless fallback is needed inside get_cpu_info
        // But get_cpu_info takes &mut ctx and might call get_cpu_info_wmi which needs WMI.
        // We should update get_cpu_info to init WMI if needed.
        let cpus = hardware::cpu::get_cpu_info(&mut ctx).map_err(|e| e.to_string())?;
        let scored_cpus: Vec<ScoredCpu> = cpus.into_iter().map(|cpu| {
            let score = scoring::score_cpu(cpu.number_of_cores, cpu.max_clock_speed);
            let score_num = scoring::calculate_cpu_score_num(cpu.number_of_cores, cpu.max_clock_speed);
            ScoredCpu {
                info: cpu,
                score: format!("{:?}", score),
                score_num,
            }
        }).collect();
        Ok::<Vec<ScoredCpu>, String>(scored_cpus)
    });

    let gpu_handle = std::thread::spawn(|| {
        let mut ctx = HardwareContext::new();
        ctx.init_wmi().map_err(|e| e.to_string())?;
        let gpus = hardware::gpu::get_gpu_info(&ctx).map_err(|e| e.to_string())?;
        let scored_gpus: Vec<ScoredGpu> = gpus.into_iter().map(|gpu| {
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
        Ok::<Vec<ScoredGpu>, String>(scored_gpus)
    });

    let ram_handle = std::thread::spawn(|| {
        let mut ctx = HardwareContext::new();
        ctx.init_wmi().map_err(|e| e.to_string())?;
        let mems = hardware::memory::get_memory_info(&mut ctx).map_err(|e| e.to_string())?;
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
        Ok::<ScoredRam, String>(ScoredRam {
            info: mems,
            total_gb,
            avg_speed,
            score: ram_score,
            score_num: ram_score_num,
        })
    });

    let disk_handle = std::thread::spawn(|| {
        let mut ctx = HardwareContext::new();
        ctx.init_wmi().map_err(|e| e.to_string())?;
        let disks = hardware::disk::get_disk_info(&ctx).map_err(|e| e.to_string())?;
        let scored_disks: Vec<ScoredDisk> = disks.into_iter().map(|disk| {
            let size = disk.size;
            let is_ssd = disk.media_type.to_lowercase().contains("ssd");
            let is_nvme = disk.bus_type.to_lowercase().contains("nvme");
            let score = scoring::score_disk(is_ssd, is_nvme, size);
            let score_num = scoring::calculate_disk_score_num(is_ssd, is_nvme, size);
            ScoredDisk {
                info: disk,
                score: format!("{:?}", score),
                score_num,
            }
        }).collect();
        Ok::<Vec<ScoredDisk>, String>(scored_disks)
    });

    let misc_handle = std::thread::spawn(|| {
        let mut ctx = HardwareContext::new();
        ctx.init_wmi().map_err(|e| e.to_string())?;
        let sound = hardware::sound::get_sound_info(&ctx).map_err(|e| e.to_string())?;
        let monitor = hardware::monitor::get_monitor_info(&ctx).unwrap_or_default();
        let network = hardware::network::get_network_info(&ctx).unwrap_or_default();
        Ok::<(Vec<hardware::sound::SoundInfo>, Vec<hardware::monitor::MonitorInfo>, Vec<hardware::network::NetworkInfo>), String>((sound, monitor, network))
    });

    let peripherals_handle = std::thread::spawn(|| {
        // let mut ctx = HardwareContext::new();
        // ctx.init_wmi().map_err(|e| e.to_string())?;
        // let usb = hardware::peripherals::get_usb_devices(&ctx).unwrap_or_default();
        // let camera = hardware::peripherals::get_camera_devices(&ctx).unwrap_or_default();
        // let bluetooth = hardware::peripherals::get_bluetooth_devices(&ctx).unwrap_or_default();
        // Ok::<(Vec<hardware::peripherals::PnPDevice>, Vec<hardware::peripherals::PnPDevice>, Vec<hardware::peripherals::PnPDevice>), String>((usb, camera, bluetooth))
        Ok::<(Vec<hardware::peripherals::PnPDevice>, Vec<hardware::peripherals::PnPDevice>, Vec<hardware::peripherals::PnPDevice>), String>((vec![], vec![], vec![]))
    });

    // Join all threads and collect results
    let motherboard = motherboard_handle.join().map_err(|_| "Motherboard thread panicked".to_string())??;
    let cpu = cpu_handle.join().map_err(|_| "CPU thread panicked".to_string())??;
    let gpu = gpu_handle.join().map_err(|_| "GPU thread panicked".to_string())??;
    let ram = ram_handle.join().map_err(|_| "RAM thread panicked".to_string())??;
    let disks = disk_handle.join().map_err(|_| "Disk thread panicked".to_string())??;
    let (sound, monitor, network) = misc_handle.join().map_err(|_| "Misc thread panicked".to_string())??;
    let (usb, camera, bluetooth) = peripherals_handle.join().map_err(|_| "Peripherals thread panicked".to_string())??;

    Ok(FullHardwareInfo {
        motherboard,
        cpu,
        gpu,
        ram,
        disks,
        sound,
        monitor,
        network,
        usb,
        camera,
        bluetooth,
    })
}

#[tauri::command]
fn get_firewall_status() -> bool {
    // 使用 netsh 进行简单检查
    let output = Command::new("netsh")
        .args(&["advfirewall", "show", "allprofiles", "state"])
        .output();
    
    if let Ok(out) = output {
        let stdout = String::from_utf8_lossy(&out.stdout);
        // 如果任何配置文件为 ON，为简单起见我们认为它是开启的，或者检查是否全部开启。
        // 输出格式: "State ON" 或 "State OFF"
        return stdout.to_lowercase().contains("on");
    }
    false
}

#[tauri::command]
fn set_firewall_status(enable: bool) -> Result<String, String> {
    let state = if enable { "on" } else { "off" };
    
    // 首先尝试 netsh
    let output = Command::new("netsh")
        .args(&["advfirewall", "set", "allprofiles", "state", state])
        .output()
        .map_err(|e| e.to_string())?;
    
    // 同时尝试注册表作为后备/增强
    let val: u32 = if enable { 1 } else { 0 };
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let profiles = ["StandardProfile", "PublicProfile", "DomainProfile"];
    
    for profile in profiles.iter() {
        let path = format!("SYSTEM\\CurrentControlSet\\Services\\SharedAccess\\Parameters\\FirewallPolicy\\{}", profile);
        if let Ok((key, _)) = hklm.create_subkey(&path) {
            let _ = key.set_value("EnableFirewall", &val);
        }
    }

    if output.status.success() {
        Ok(format!("Firewall turned {}", state))
    } else {
        // 如果 netsh 失败，但我们尝试了注册表，我们可以报告成功并带有警告或直接报错。
        // 但通常如果 netsh 失败，那是权限问题，所以注册表也可能会失败，除非我们是 SYSTEM。
        // 但是，如果失败，我们将返回 netsh 错误，因为它是最可靠的指标。
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
    // 如果未设置，默认为启用
    true
}

#[tauri::command]
fn set_cortana_status(enable: bool) -> Result<String, String> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let (key, _) = hklm.create_subkey("SOFTWARE\\Policies\\Microsoft\\Windows\\Windows Search")
        .map_err(|e| e.to_string())?;
    
    let val: u32 = if enable { 1 } else { 0 };
    key.set_value("AllowCortana", &val).map_err(|e| e.to_string())?;
    key.set_value("AllowCloudSearch", &val).map_err(|e| e.to_string())?;
    key.set_value("ConnectedSearchUseWeb", &val).map_err(|e| e.to_string())?;
    
    // 同时尝试从任务栏隐藏 (HKCU)
    if !enable {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        if let Ok((search_key, _)) = hkcu.create_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Search") {
            let _ = search_key.set_value("SearchboxTaskbarMode", &0u32); // 0 = 隐藏
        }
    }

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

    // 过滤 NVIDIA 或 AMD
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
        // 要恢复默认设置，我们删除该键
        // 如果父 CLSID 键为空，我们需要删除它，但目前只删除 InprocServer32 就足够了，或者删除 CLSID 本身
        let _ = hkcu.delete_subkey_all("Software\\Classes\\CLSID\\{86ca1aa0-34aa-4e8b-a509-50c905bae2a2}");
    }
    Ok("Success".to_string())
}

#[tauri::command]
fn set_show_extensions(enable: bool) -> Result<String, String> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let advanced = hkcu.open_subkey_with_flags("Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced", KEY_WRITE).map_err(|e| e.to_string())?;
    
    // HideFileExt: 0 = 显示, 1 = 隐藏
    let val = if enable { 0u32 } else { 1u32 };
    advanced.set_value("HideFileExt", &val).map_err(|e| e.to_string())?;
    
    Ok("Success".to_string())
}

#[tauri::command]
fn set_show_hidden_files(enable: bool) -> Result<String, String> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let advanced = hkcu.open_subkey_with_flags("Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced", KEY_WRITE).map_err(|e| e.to_string())?;
    
    // Hidden: 1 = 显示, 2 = 隐藏
    let val = if enable { 1u32 } else { 2u32 };
    advanced.set_value("Hidden", &val).map_err(|e| e.to_string())?;
    
    Ok("Success".to_string())
}

#[tauri::command]
fn restart_explorer() -> Result<String, String> {
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x08000000;

    // 终止资源管理器
    let _ = Command::new("taskkill")
        .args(&["/F", "/IM", "explorer.exe"])
        .creation_flags(CREATE_NO_WINDOW)
        .output();

    // 启动资源管理器
    Command::new("explorer.exe")
        .creation_flags(CREATE_NO_WINDOW)
        .spawn()
        .map_err(|e| e.to_string())?;

    Ok("Success".to_string())
}

#[tauri::command]
async fn quick_memory_check(window: tauri::Window) -> Result<String, String> {
    // Allocate 100MB chunks and test
    let chunk_size = 100 * 1024 * 1024; // 100MB
    let chunks_to_test = 10; // Test 1GB total (Increased for better demo)
    
    for i in 0..chunks_to_test {
        // Emit progress event
        let progress = ((i as f32 / chunks_to_test as f32) * 100.0) as u32;
        window.emit("memory_progress", progress).unwrap_or(());

        // Simulate some work/delay to make it visible and not freeze UI if it was sync (though this is async now)
        // In a real heavy test, we might want to spawn a thread, but async here allows event loop to process
        std::thread::sleep(std::time::Duration::from_millis(200));

        let mut buffer: Vec<u8> = Vec::with_capacity(chunk_size);
        // Write pattern
        for _ in 0..chunk_size {
            buffer.push(0xAA);
        }
        
        // Verify pattern
        for (index, byte) in buffer.iter().enumerate() {
            if *byte != 0xAA {
                return Err(format!("Memory error at chunk {}, byte {}", i, index));
            }
        }
        
        // Write another pattern
        for j in 0..chunk_size {
            buffer[j] = 0x55;
        }
        
        // Verify
        for (index, byte) in buffer.iter().enumerate() {
            if *byte != 0x55 {
                return Err(format!("Memory error at chunk {}, byte {}", i, index));
            }
        }
        
        // Drop to free memory
        drop(buffer);
    }
    
    window.emit("memory_progress", 100).unwrap_or(());
    Ok("Memory check passed (Quick Test)".to_string())
}

#[tauri::command]
fn get_uptime() -> u64 {
    System::uptime()
}

#[tauri::command]
fn run_memory_diagnostic() -> Result<String, String> {
    // mdsched.exe 是 Windows 内存诊断工具
    // 它通常需要管理员权限并显示 GUI
    // 使用 cmd /c start 确保它正确处理 UAC/GUI
    Command::new("cmd")
        .args(&["/C", "start", "mdsched.exe"])
        .spawn()
        .map_err(|e| e.to_string())?;

    Ok("Success".to_string())
}

#[tauri::command]
fn open_color_cpl() -> Result<String, String> {
    Command::new("colorcpl.exe")
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok("Success".to_string())
}

#[tauri::command]
fn set_autostart(enable: bool) -> Result<(), String> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = r"Software\Microsoft\Windows\CurrentVersion\Run";
    let (key, _) = hkcu.create_subkey(path).map_err(|e| e.to_string())?;
    
    let app_name = "Neko233HardwareViewer";
    
    if enable {
        let exe_path = std::env::current_exe().map_err(|e| e.to_string())?;
        key.set_value(app_name, &exe_path.to_string_lossy().as_ref()).map_err(|e| e.to_string())?;
    } else {
        let _ = key.delete_value(app_name);
    }
    Ok(())
}

#[tauri::command]
fn check_autostart() -> bool {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = r"Software\Microsoft\Windows\CurrentVersion\Run";
    if let Ok(key) = hkcu.open_subkey(path) {
        let app_name = "Neko233HardwareViewer";
        return key.get_value::<String, _>(app_name).is_ok();
    }
    false
}

#[tauri::command]
fn is_game_running() -> bool {
    use winapi::um::winuser::{GetForegroundWindow, GetWindowRect, GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};
    use winapi::shared::windef::RECT;
    
    unsafe {
        let hwnd = GetForegroundWindow();
        if hwnd.is_null() { return false; }
        
        let mut rect: RECT = std::mem::zeroed();
        if GetWindowRect(hwnd, &mut rect) == 0 { return false; }
        
        let screen_w = GetSystemMetrics(SM_CXSCREEN);
        let screen_h = GetSystemMetrics(SM_CYSCREEN);
        
        let win_w = rect.right - rect.left;
        let win_h = rect.bottom - rect.top;
        
        if win_w >= screen_w && win_h >= screen_h {
            return true;
        }
    }
    false
}

#[tauri::command]
fn optimize_processor() -> Result<String, String> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    
    // SystemResponsiveness -> 0
    let path_profile = "SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile";
    if let Ok((key, _)) = hklm.create_subkey(path_profile) {
        let _ = key.set_value("SystemResponsiveness", &0u32);
    }

    // PowerThrottlingOff -> 1
    let path_throttling = "SYSTEM\\CurrentControlSet\\Control\\Power\\PowerThrottling";
    if let Ok((key, _)) = hklm.create_subkey(path_throttling) {
        let _ = key.set_value("PowerThrottlingOff", &1u32);
    }

    Ok("Processor optimization applied (SystemResponsiveness=0, PowerThrottlingOff=1)".to_string())
}

#[tauri::command]
fn enable_high_perf_plan() -> Result<String, String> {
    // Try to duplicate Ultimate Performance scheme
    let output = Command::new("powercfg")
        .args(&["-duplicatescheme", "e9a42b02-d5df-448d-aa00-03f14749eb61"])
        .output()
        .map_err(|e| e.to_string())?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Extract GUID from output: "Power Scheme GUID: <GUID>  (Ultimate Performance)"
    let guid = stdout.split_whitespace()
        .find(|&s| s.len() == 36 && s.contains('-')) // Simple GUID check
        .unwrap_or("8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c"); // Fallback to High Performance

    // Set active
    Command::new("powercfg")
        .args(&["-setactive", guid])
        .output()
        .map_err(|e| e.to_string())?;

    Ok(format!("Power plan set to High/Ultimate Performance ({})", guid))
}

#[tauri::command]
fn increase_fs_cache() -> Result<String, String> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let path = "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Memory Management";
    
    if let Ok((key, _)) = hklm.create_subkey(path) {
        // Set IoPageLockLimit to 64MB (67108864 bytes)
        // This helps with file transfer speeds on systems with plenty of RAM
        let val: u32 = 67108864; 
        let _ = key.set_value("IoPageLockLimit", &val);
        Ok("File System Cache increased (IoPageLockLimit=64MB)".to_string())
    } else {
        Err("Failed to access registry key".to_string())
    }
}

#[tauri::command]
fn enable_large_system_cache() -> Result<String, String> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let path = "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Memory Management";
    
    if let Ok((key, _)) = hklm.create_subkey(path) {
        let _ = key.set_value("LargeSystemCache", &1u32);
        Ok("Large System Cache enabled".to_string())
    } else {
        Err("Failed to access registry key".to_string())
    }
}

#[derive(Serialize)]
struct PeripheralsInfo {
    usb: Vec<hardware::peripherals::PnPDevice>,
    camera: Vec<hardware::peripherals::PnPDevice>,
    bluetooth: Vec<hardware::peripherals::PnPDevice>,
}

#[tauri::command]
fn get_motherboard_info_command() -> Result<Vec<hardware::motherboard::MotherboardInfo>, String> {
    let mut ctx = HardwareContext::new();
    ctx.init_wmi().map_err(|e| e.to_string())?;
    hardware::motherboard::get_motherboard_info(&ctx).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_cpu_info_command() -> Result<Vec<ScoredCpu>, String> {
    let mut ctx = HardwareContext::new();
    let cpus = hardware::cpu::get_cpu_info(&mut ctx).map_err(|e| e.to_string())?;
    let scored_cpus: Vec<ScoredCpu> = cpus.into_iter().map(|cpu| {
        let score = scoring::score_cpu(cpu.number_of_cores, cpu.max_clock_speed);
        let score_num = scoring::calculate_cpu_score_num(cpu.number_of_cores, cpu.max_clock_speed);
        ScoredCpu {
            info: cpu,
            score: format!("{:?}", score),
            score_num,
        }
    }).collect();
    Ok(scored_cpus)
}

#[tauri::command]
fn get_gpu_info_command() -> Result<Vec<ScoredGpu>, String> {
    let mut ctx = HardwareContext::new();
    ctx.init_wmi().map_err(|e| e.to_string())?;
    let gpus = hardware::gpu::get_gpu_info(&ctx).map_err(|e| e.to_string())?;
    let scored_gpus: Vec<ScoredGpu> = gpus.into_iter().map(|gpu| {
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
    Ok(scored_gpus)
}

#[tauri::command]
fn get_ram_info_command() -> Result<ScoredRam, String> {
    let mut ctx = HardwareContext::new();
    ctx.init_wmi().map_err(|e| e.to_string())?;
    let mems = hardware::memory::get_memory_info(&mut ctx).map_err(|e| e.to_string())?;
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
    Ok(ScoredRam {
        info: mems,
        total_gb,
        avg_speed,
        score: ram_score,
        score_num: ram_score_num,
    })
}

#[tauri::command]
fn get_disk_info_command() -> Result<Vec<ScoredDisk>, String> {
    let mut ctx = HardwareContext::new();
    ctx.init_wmi().map_err(|e| e.to_string())?;
    let disks = hardware::disk::get_disk_info(&ctx).map_err(|e| e.to_string())?;
    let scored_disks: Vec<ScoredDisk> = disks.into_iter().map(|disk| {
        let size = disk.size;
        let is_ssd = disk.media_type.to_lowercase().contains("ssd");
        let is_nvme = disk.bus_type.to_lowercase().contains("nvme");
        let score = scoring::score_disk(is_ssd, is_nvme, size);
        let score_num = scoring::calculate_disk_score_num(is_ssd, is_nvme, size);
        ScoredDisk {
            info: disk,
            score: format!("{:?}", score),
            score_num,
        }
    }).collect();
    Ok(scored_disks)
}

#[tauri::command]
fn get_sound_info_command() -> Result<Vec<hardware::sound::SoundInfo>, String> {
    let mut ctx = HardwareContext::new();
    ctx.init_wmi().map_err(|e| e.to_string())?;
    hardware::sound::get_sound_info(&ctx).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_monitor_info_command() -> Result<Vec<hardware::monitor::MonitorInfo>, String> {
    let mut ctx = HardwareContext::new();
    ctx.init_wmi().map_err(|e| e.to_string())?;
    hardware::monitor::get_monitor_info(&ctx).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_network_info_command() -> Result<Vec<hardware::network::NetworkInfo>, String> {
    let mut ctx = HardwareContext::new();
    ctx.init_wmi().map_err(|e| e.to_string())?;
    hardware::network::get_network_info(&ctx).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_peripherals_info_command() -> Result<PeripheralsInfo, String> {
    let mut ctx = HardwareContext::new();
    ctx.init_wmi().map_err(|e| e.to_string())?;
    let usb = hardware::peripherals::get_usb_devices(&ctx).map_err(|e| e.to_string())?;
    let camera = hardware::peripherals::get_camera_devices(&ctx).map_err(|e| e.to_string())?;
    let bluetooth = hardware::peripherals::get_bluetooth_devices(&ctx).map_err(|e| e.to_string())?;
    Ok(PeripheralsInfo { usb, camera, bluetooth })
}

mod optimization;
mod diagnostics;
mod apps;
mod network_tools;

fn main() {
    let sys = System::new_all();
    let app_state = AppState {
        sys: Mutex::new(sys),
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            get_hardware_info, 
            get_system_usage,
            get_boot_time,
            get_uptime,
            get_firewall_status,
            save_tab_order,
            load_tab_order,
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
            run_memory_diagnostic,
            quick_memory_check,
            open_color_cpl,
            set_autostart,
            check_autostart,
            is_game_running,
            optimize_processor,
            enable_high_perf_plan,
            increase_fs_cache,
            enable_large_system_cache,
            // Individual Hardware Commands
            get_motherboard_info_command,
            get_cpu_info_command,
            get_gpu_info_command,
            get_ram_info_command,
            get_disk_info_command,
            get_sound_info_command,
            get_monitor_info_command,
            get_network_info_command,
            get_peripherals_info_command,
            // Optimization
            optimization::apply_optimization,
            optimization::disable_telemetry,
            optimization::disable_windows_update,
            optimization::disable_hibernation,
            optimization::disable_game_dvr,
            optimization::disable_sticky_keys,
            optimization::disable_mouse_acceleration,
            optimization::disable_lock_screen,
            optimization::disable_bing_search,
            optimization::disable_aero_shake,
            optimization::disable_timeline,
            optimization::clear_temp_files,
            optimization::clear_dns_cache,
            optimization::reset_network_stack,
            // Diagnostics
            diagnostics::check_disk_health,
            diagnostics::check_system_file_integrity,
            diagnostics::check_dism_health,
            diagnostics::check_battery_health,
            diagnostics::check_network_latency,
            diagnostics::check_dns_resolution,
            diagnostics::check_activation_status_detailed,
            diagnostics::check_tpm_status,
            diagnostics::check_secure_boot_status,
            diagnostics::check_antivirus_status,
            // Apps
            apps::get_installed_apps,
            apps::uninstall_app,
            // Network Tools
            network_tools::get_network_interfaces_detailed,
            network_tools::ping_hosts,
            network_tools::get_public_ip,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
