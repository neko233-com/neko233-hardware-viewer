pub mod motherboard;
pub mod cpu;
pub mod gpu;
pub mod memory;
pub mod disk;
pub mod sound;

use wmi::{COMLibrary, WMIConnection, WMIError};
use anyhow::Result;

pub struct HardwareContext {
    pub wmi_con: WMIConnection,
}

impl HardwareContext {
    pub fn new() -> Result<Self> {
        let com_con = COMLibrary::new().or_else(|_| {
            unsafe { Ok::<COMLibrary, WMIError>(COMLibrary::assume_initialized()) }
        })?;
        let wmi_con = WMIConnection::new(com_con)?;
        Ok(Self { wmi_con })
    }
}
