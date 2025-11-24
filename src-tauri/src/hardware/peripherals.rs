use serde::{Deserialize, Serialize};
use super::HardwareContext;
use anyhow::Result;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct PnPDevice {
    pub name: String,
    pub manufacturer: Option<String>,
    pub status: Option<String>,
    pub pnp_class: Option<String>,
}

pub fn get_usb_devices(ctx: &HardwareContext) -> Result<Vec<PnPDevice>> {
    // PNPClass = 'USB' gets controllers and hubs.
    // To get connected devices, we might need to look broader, but let's start with USB class.
    let wmi = ctx.get_wmi()?;
    let results: Vec<PnPDevice> = wmi.raw_query("SELECT Name, Manufacturer, Status, PNPClass FROM Win32_PnPEntity WHERE PNPClass = 'USB'")?;
    Ok(results)
}

pub fn get_camera_devices(ctx: &HardwareContext) -> Result<Vec<PnPDevice>> {
    // Cameras are usually 'Camera' or 'Image' class
    let wmi = ctx.get_wmi()?;
    let results: Vec<PnPDevice> = wmi.raw_query("SELECT Name, Manufacturer, Status, PNPClass FROM Win32_PnPEntity WHERE PNPClass = 'Camera' OR PNPClass = 'Image'")?;
    Ok(results)
}

pub fn get_bluetooth_devices(ctx: &HardwareContext) -> Result<Vec<PnPDevice>> {
    let wmi = ctx.get_wmi()?;
    let results: Vec<PnPDevice> = wmi.raw_query("SELECT Name, Manufacturer, Status, PNPClass FROM Win32_PnPEntity WHERE PNPClass = 'Bluetooth'")?;
    Ok(results)
}
