use serde::{Deserialize, Serialize};
use super::HardwareContext;
use anyhow::Result;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct CpuInfo {
    pub name: String,
    pub max_clock_speed: u32,
    pub number_of_cores: u32,
    pub number_of_logical_processors: u32,
    pub manufacturer: String,
}

pub fn get_cpu_info(ctx: &HardwareContext) -> Result<Vec<CpuInfo>> {
    let results: Vec<CpuInfo> = ctx.wmi_con.raw_query("SELECT * FROM Win32_Processor")?;
    Ok(results)
}
