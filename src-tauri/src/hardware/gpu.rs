use serde::{Deserialize, Serialize};
use super::HardwareContext;
use anyhow::Result;
use winreg::enums::*;
use winreg::RegKey;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct GpuInfo {
    pub name: String,
    pub driver_version: String,
    pub adapter_ram: Option<u64>,
    pub video_processor: Option<String>,
    pub adapter_compatibility: Option<String>,
    pub driver_date: Option<String>,
    pub video_mode_description: Option<String>,
    pub current_refresh_rate: Option<u32>,
    pub current_horizontal_resolution: Option<u32>,
    pub current_vertical_resolution: Option<u32>,
}

fn get_registry_vram(driver_desc: &str) -> Option<u64> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let video_class_path = "SYSTEM\\CurrentControlSet\\Control\\Class\\{4d36e968-e325-11ce-bfc1-08002be10318}";
    
    if let Ok(class_key) = hklm.open_subkey(video_class_path) {
        for i in 0..20 { // Check 0000 to 0019
            let subkey_name = format!("{:04}", i);
            if let Ok(subkey) = class_key.open_subkey(&subkey_name) {
                // Check if DriverDesc matches
                if let Ok(desc) = subkey.get_value::<String, _>("DriverDesc") {
                    if desc == driver_desc {
                        // Try qwMemorySize (u64)
                        if let Ok(size) = subkey.get_value::<u64, _>("HardwareInformation.qwMemorySize") {
                            return Some(size);
                        }
                        // Try MemorySize (u32)
                        if let Ok(size) = subkey.get_value::<u32, _>("HardwareInformation.MemorySize") {
                            return Some(size as u64);
                        }
                    }
                }
            }
        }
    }
    None
}

pub fn get_gpu_info(ctx: &HardwareContext) -> Result<Vec<GpuInfo>> {
    let wmi = ctx.get_wmi()?;
    let mut results: Vec<GpuInfo> = wmi.raw_query("SELECT Name, DriverVersion, AdapterRAM, VideoProcessor, AdapterCompatibility, DriverDate, VideoModeDescription, CurrentRefreshRate, CurrentHorizontalResolution, CurrentVerticalResolution FROM Win32_VideoController")?;
    
    for gpu in &mut results {
        // If AdapterRAM is missing or small (likely wrong for dedicated GPU), try registry
        // 1GB = 1073741824 bytes. If < 1GB, it might be wrong or iGPU.
        // We try registry to see if we can get a better value.
        if gpu.adapter_ram.unwrap_or(0) < 1073741824 { 
             if let Some(vram) = get_registry_vram(&gpu.name) {
                 // Only update if registry value is larger (more likely to be correct for dGPU)
                 if vram > gpu.adapter_ram.unwrap_or(0) {
                     gpu.adapter_ram = Some(vram);
                 }
             } else {
                // Fallback: Try to guess from Name (Very rough, but better than 0)
                // This is a last resort for display purposes
                if gpu.name.contains("RTX 4090") { gpu.adapter_ram = Some(24 * 1024 * 1024 * 1024); }
                else if gpu.name.contains("RTX 4080") { gpu.adapter_ram = Some(16 * 1024 * 1024 * 1024); }
                else if gpu.name.contains("RTX 4070 Ti") { gpu.adapter_ram = Some(12 * 1024 * 1024 * 1024); }
                else if gpu.name.contains("RTX 4070") { gpu.adapter_ram = Some(12 * 1024 * 1024 * 1024); }
                else if gpu.name.contains("RTX 3090") { gpu.adapter_ram = Some(24 * 1024 * 1024 * 1024); }
                else if gpu.name.contains("RTX 3080") { gpu.adapter_ram = Some(10 * 1024 * 1024 * 1024); }
                else if gpu.name.contains("RTX 3070") { gpu.adapter_ram = Some(8 * 1024 * 1024 * 1024); }
                else if gpu.name.contains("RTX 3060") { gpu.adapter_ram = Some(12 * 1024 * 1024 * 1024); }
             }
        }
    }
    
    Ok(results)
}
