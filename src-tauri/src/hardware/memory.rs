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

pub fn get_memory_info(ctx: &HardwareContext) -> Result<Vec<MemoryInfo>> {
    let results: Vec<MemoryInfo> = ctx.wmi_con.raw_query("SELECT * FROM Win32_PhysicalMemory")?;
    Ok(results)
}
