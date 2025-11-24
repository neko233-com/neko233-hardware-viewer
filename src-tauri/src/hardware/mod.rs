pub mod motherboard;
pub mod cpu;
pub mod gpu;
pub mod memory;
pub mod disk;
pub mod sound;
pub mod monitor;
pub mod network;
pub mod peripherals;

use wmi::{COMLibrary, WMIConnection, WMIError};
use anyhow::Result;
use sysinfo::System;

pub struct HardwareContext {
    pub wmi_con: Option<WMIConnection>,
    pub sys: System,
}

impl HardwareContext {
    pub fn new() -> Self {
        let sys = System::new();
        Self { wmi_con: None, sys }
    }

    pub fn init_wmi(&mut self) -> Result<()> {
        if self.wmi_con.is_some() {
            return Ok(());
        }
        let com_con = COMLibrary::new().or_else(|_| {
            unsafe { Ok::<COMLibrary, WMIError>(COMLibrary::assume_initialized()) }
        })?;
        let wmi_con = WMIConnection::new(com_con)?;
        self.wmi_con = Some(wmi_con);
        Ok(())
    }
    
    pub fn get_wmi(&self) -> Result<&WMIConnection> {
        self.wmi_con.as_ref().ok_or_else(|| anyhow::anyhow!("WMI not initialized"))
    }
}
