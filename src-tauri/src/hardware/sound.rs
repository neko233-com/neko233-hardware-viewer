use serde::{Deserialize, Serialize};
use super::HardwareContext;
use anyhow::Result;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct SoundInfo {
    pub name: String,
    pub manufacturer: Option<String>,
    pub status: Option<String>,
}

pub fn get_sound_info(ctx: &HardwareContext) -> Result<Vec<SoundInfo>> {
    let wmi = ctx.get_wmi()?;
    let results: Vec<SoundInfo> = wmi.raw_query("SELECT * FROM Win32_SoundDevice")?;
    Ok(results)
}
