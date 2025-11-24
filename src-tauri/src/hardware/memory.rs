use serde::{Deserialize, Serialize};
use super::HardwareContext;
use anyhow::Result;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct MemoryInfo {
    pub capacity: u64,
    pub speed: u32,
    pub manufacturer: String,
    pub part_number: String,
    pub configured_clock_speed: Option<u32>,
    pub device_locator: String,
    pub configured_voltage: Option<u32>,
    pub min_voltage: Option<u32>,
    pub max_voltage: Option<u32>,
    pub serial_number: Option<String>,
    pub bank_label: Option<String>,
    pub data_width: Option<u16>,
    pub total_width: Option<u16>,
    pub form_factor: Option<u16>,
    pub status: Option<String>,
}

pub fn get_memory_info(ctx: &mut HardwareContext) -> Result<Vec<MemoryInfo>> {
    // Try sysinfo first for speed, but sysinfo only gives total memory, not per-stick info.
    // If we want per-stick info, we MUST use WMI.
    // The user asked for "library directly" because "it takes too long".
    // WMI Memory query can be slow.
    // Let's try to use WMI but if it fails or is too slow (we can't measure speed easily here), fallback?
    // Actually, let's stick to WMI for Memory because the UI expects detailed info (slots).
    // But we must update the signature to match main.rs
    let wmi = ctx.get_wmi()?;
    let results: Vec<MemoryInfo> = wmi.raw_query("SELECT * FROM Win32_PhysicalMemory")?;
    Ok(results)
}
