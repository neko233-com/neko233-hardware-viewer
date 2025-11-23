#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod hardware;
mod scoring;

use hardware::HardwareContext;
use serde::Serialize;

// Define structs for the frontend response
#[derive(Serialize)]
struct ScoredCpu {
    info: hardware::cpu::CpuInfo,
    score: String,
}

#[derive(Serialize)]
struct ScoredGpu {
    info: hardware::gpu::GpuInfo,
    score: String,
}

#[derive(Serialize)]
struct ScoredRam {
    info: Vec<hardware::memory::MemoryInfo>,
    total_gb: u64,
    avg_speed: u32,
    score: String,
}

#[derive(Serialize)]
struct ScoredDisk {
    info: hardware::disk::DiskInfo,
    score: String,
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
        ScoredCpu {
            info: cpu,
            score: format!("{:?}", score),
        }
    }).collect();

    let gpus = hardware::gpu::get_gpu_info(&ctx).map_err(|e| e.to_string())?;
    let scored_gpus = gpus.into_iter().map(|gpu| {
        let score = if let Some(ram) = gpu.adapter_ram {
             format!("{:?}", scoring::score_gpu(ram))
        } else {
             "Unknown".to_string()
        };
        ScoredGpu {
            info: gpu,
            score,
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
    let scored_ram = ScoredRam {
        info: mems,
        total_gb,
        avg_speed,
        score: ram_score,
    };

    let disks = hardware::disk::get_disk_info(&ctx).map_err(|e| e.to_string())?;
    let scored_disks = disks.into_iter().map(|disk| {
        let score = if let Some(size) = disk.size {
            format!("{:?}", scoring::score_disk(size))
        } else {
            "Unknown".to_string()
        };
        ScoredDisk {
            info: disk,
            score,
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

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_hardware_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
