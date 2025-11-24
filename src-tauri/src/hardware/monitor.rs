use serde::{Deserialize, Serialize};
use super::HardwareContext;
use anyhow::Result;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct MonitorInfo {
    pub name: String,
    pub manufacturer: Option<String>,
    pub screen_height: Option<u32>,
    pub screen_width: Option<u32>,
}

pub fn get_monitor_info(ctx: &HardwareContext) -> Result<Vec<MonitorInfo>> {
    // PowerShell is too slow. Reverting to WMI Win32_DesktopMonitor.
    // Names will be generic but it's fast.
    let wmi = ctx.get_wmi()?;
    let monitors: Vec<WmiMonitor> = wmi.raw_query("SELECT Name, MonitorManufacturer, ScreenHeight, ScreenWidth FROM Win32_DesktopMonitor")?;
    
    let mut results = Vec::new();
    for m in monitors {
        results.push(MonitorInfo {
            name: m.name.unwrap_or("Generic Monitor".to_string()),
            manufacturer: m.monitor_manufacturer,
            screen_height: m.screen_height,
            screen_width: m.screen_width,
        });
    }
    Ok(results)
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct WmiMonitor {
    name: Option<String>,
    monitor_manufacturer: Option<String>,
    screen_height: Option<u32>,
    screen_width: Option<u32>,
}
