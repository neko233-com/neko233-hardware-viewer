use serde::{Deserialize, Serialize};
use super::HardwareContext;
use anyhow::Result;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct MotherboardInfo {
    pub manufacturer: String,
    pub product: String,
    pub version: String,
    pub serial_number: String,
}

pub fn get_motherboard_info(ctx: &HardwareContext) -> Result<Vec<MotherboardInfo>> {
    let results: Vec<MotherboardInfo> = ctx.wmi_con.raw_query("SELECT * FROM Win32_BaseBoard")?;
    Ok(results)
}
