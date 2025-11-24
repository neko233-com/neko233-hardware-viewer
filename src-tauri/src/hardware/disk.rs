use serde::{Deserialize, Serialize};
use super::HardwareContext;
use anyhow::Result;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct DiskInfo {
    pub model: String,
    pub size: u64,
    pub media_type: String,
    pub bus_type: String,
    pub health_status: String,
    pub operational_status: String,
    pub serial_number: String,
    pub firmware_revision: String,
    pub partitions: Option<u32>,
    pub status: Option<String>,
    pub pcie_profile: Option<String>,
}

pub fn get_disk_info(ctx: &HardwareContext) -> Result<Vec<DiskInfo>> {
    // PowerShell is too slow (50x slower). Reverting to WMI.
    // We lose PCIe info but gain speed.
    let wmi = ctx.get_wmi()?;
    let disks: Vec<WmiDisk> = wmi.raw_query("SELECT Model, Size, MediaType, InterfaceType, Status, SerialNumber, FirmwareRevision, Partitions FROM Win32_DiskDrive")?;
    
    let mut results = Vec::new();
    for d in disks {
        results.push(DiskInfo {
            model: d.model,
            size: d.size,
            media_type: d.media_type.unwrap_or("Unknown".to_string()),
            bus_type: d.interface_type.unwrap_or("Unknown".to_string()),
            health_status: d.status.clone().unwrap_or("Unknown".to_string()),
            operational_status: "OK".to_string(),
            serial_number: d.serial_number.unwrap_or_default(),
            firmware_revision: d.firmware_revision.unwrap_or_default(),
            partitions: d.partitions,
            status: d.status,
            pcie_profile: None,
        });
    }
    Ok(results)
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct WmiDisk {
    model: String,
    size: u64,
    media_type: Option<String>,
    interface_type: Option<String>,
    status: Option<String>,
    serial_number: Option<String>,
    firmware_revision: Option<String>,
    partitions: Option<u32>,
}
