use serde::{Deserialize, Serialize};
use super::HardwareContext;
use anyhow::Result;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct NetworkInfo {
    pub name: String,
    pub manufacturer: Option<String>,
    pub adapter_type: Option<String>,
    pub net_connection_id: Option<String>,
    pub speed: Option<u64>,
    pub mac_address: Option<String>,
    pub net_connection_status: Option<u16>, // 2=Connected
}

pub fn get_network_info(ctx: &HardwareContext) -> Result<Vec<NetworkInfo>> {
    // Filter for physical adapters (AdapterTypeID=0 is Ethernet 802.3)
    // But WiFi is also important.
    // We filter where NetConnectionID is not null to avoid virtual adapters like WAN Miniport
    let wmi = ctx.get_wmi()?;
    let results: Vec<NetworkInfo> = wmi.raw_query("SELECT Name, Manufacturer, AdapterType, NetConnectionID, Speed, MACAddress, NetConnectionStatus FROM Win32_NetworkAdapter WHERE NetConnectionID IS NOT NULL")?;
    Ok(results)
}
