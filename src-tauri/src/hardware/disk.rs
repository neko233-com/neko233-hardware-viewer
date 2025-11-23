use serde::{Deserialize, Serialize};
use super::HardwareContext;
use anyhow::Result;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct DiskInfo {
    pub model: String,
    pub size: Option<u64>,
    pub media_type: Option<String>,
    pub interface_type: Option<String>,
}

pub fn get_disk_info(ctx: &HardwareContext) -> Result<Vec<DiskInfo>> {
    let results: Vec<DiskInfo> = ctx.wmi_con.raw_query("SELECT * FROM Win32_DiskDrive")?;
    Ok(results)
}
