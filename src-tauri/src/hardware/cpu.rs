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
    pub l2_cache_size: Option<u32>,
    pub l3_cache_size: Option<u32>,
    pub socket_designation: Option<String>,
    pub description: Option<String>,
    pub virtualization_firmware_enabled: Option<bool>,
}

pub fn get_cpu_info(ctx: &mut HardwareContext) -> Result<Vec<CpuInfo>> {
    ctx.sys.refresh_cpu();
    let cpus = ctx.sys.cpus();
    
    if cpus.is_empty() {
        // Fallback to WMI if sysinfo fails or returns empty
        ctx.init_wmi().map_err(|e| anyhow::anyhow!(e.to_string()))?;
        return get_cpu_info_wmi(ctx);
    }

    let first_cpu = &cpus[0];
    
    // sysinfo gives individual cores as "CPUs". We need to aggregate or just take the first one as representative for the package.
    // Assuming single socket for simplicity or we can try to group.
    // For now, let's return one CpuInfo representing the package.
    
    let info = CpuInfo {
        name: first_cpu.brand().to_string(),
        max_clock_speed: first_cpu.frequency() as u32, // This is current frequency, not max? sysinfo 0.30 frequency() is current.
        number_of_cores: ctx.sys.physical_core_count().unwrap_or(cpus.len()) as u32,
        number_of_logical_processors: cpus.len() as u32,
        manufacturer: first_cpu.vendor_id().to_string(),
        l2_cache_size: None, // sysinfo doesn't provide this
        l3_cache_size: None,
        socket_designation: None,
        description: None,
        virtualization_firmware_enabled: None,
    };

    Ok(vec![info])
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct CacheMemory {
    level: u16,
    max_cache_size: Option<u32>,
}

fn get_cpu_info_wmi(ctx: &HardwareContext) -> Result<Vec<CpuInfo>> {
    let wmi = ctx.get_wmi()?;
    let mut results: Vec<CpuInfo> = wmi.raw_query("SELECT Name, MaxClockSpeed, NumberOfCores, NumberOfLogicalProcessors, Manufacturer, L2CacheSize, L3CacheSize, SocketDesignation, Description, VirtualizationFirmwareEnabled FROM Win32_Processor")?;
    
    // Fallback for Cache Sizes if missing
    let needs_cache = results.iter().any(|cpu| cpu.l2_cache_size.is_none() || cpu.l3_cache_size.is_none() || cpu.l3_cache_size == Some(0));
    
    if needs_cache {
        if let Ok(caches) = wmi.raw_query::<CacheMemory>("SELECT Level, MaxCacheSize FROM Win32_CacheMemory") {
            let mut sorted_caches: Vec<u32> = caches.iter().filter_map(|c| c.max_cache_size).collect();
            sorted_caches.sort_by(|a, b| b.cmp(a)); // Descending
            
            if let Some(largest) = sorted_caches.first() {
                 for cpu in &mut results {
                     if cpu.l3_cache_size.unwrap_or(0) == 0 {
                         cpu.l3_cache_size = Some(*largest);
                     }
                 }
            }
             if let Some(second) = sorted_caches.get(1) {
                 for cpu in &mut results {
                     if cpu.l2_cache_size.unwrap_or(0) == 0 {
                         cpu.l2_cache_size = Some(*second);
                     }
                 }
            }
        }
    }

    Ok(results)
}
