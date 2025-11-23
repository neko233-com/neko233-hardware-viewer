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
}

pub fn get_memory_info(ctx: &HardwareContext) -> Result<Vec<MemoryInfo>> {
    let results: Vec<MemoryInfo> = ctx.wmi_con.raw_query("SELECT * FROM Win32_PhysicalMemory")?;
    Ok(results)
}
