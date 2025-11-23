use serde::{Deserialize, Serialize};
use super::HardwareContext;
use anyhow::Result;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct GpuInfo {
    pub name: String,
    pub driver_version: String,
    pub adapter_ram: Option<u64>,
    pub video_processor: Option<String>,
    pub adapter_compatibility: Option<String>, // Manufacturer often here
}

pub fn get_gpu_info(ctx: &HardwareContext) -> Result<Vec<GpuInfo>> {
    let results: Vec<GpuInfo> = ctx.wmi_con.raw_query("SELECT * FROM Win32_VideoController")?;
    Ok(results)
}
