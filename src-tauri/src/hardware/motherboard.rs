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
    #[serde(skip_deserializing, default)]
    pub chipset: String,
    #[serde(skip_deserializing, default)]
    pub ssd_slots: SlotInfo,
    #[serde(skip_deserializing, default)]
    pub gpu_slots: SlotInfo,
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct SlotInfo {
    pub total: u32,
    pub used: u32,
    pub details: Vec<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct SystemSlot {
    slot_designation: Option<String>,
    current_usage: Option<u16>, // 3=Available, 4=In Use
    description: Option<String>,
}

pub fn get_motherboard_info(ctx: &HardwareContext) -> Result<Vec<MotherboardInfo>> {
    let mut boards: Vec<MotherboardInfo> = ctx.wmi_con.raw_query("SELECT * FROM Win32_BaseBoard")?;
    
    // Fetch Slots - Use unwrap_or_default to handle cases where Win32_SystemSlot is missing or fails
    let slots: Vec<SystemSlot> = ctx.wmi_con.raw_query("SELECT SlotDesignation, CurrentUsage, Description FROM Win32_SystemSlot").unwrap_or_default();

    for board in &mut boards {
        // 1. Extract Chipset from Product (Simple Heuristic)
        // e.g. "X670E GAMING PLUS" -> "X670E"
        // Common chipsets: X670, B650, Z790, B760, Z690, B660, X570, B550, etc.
        let product_upper = board.product.to_uppercase();
        let chipsets = vec![
            "X870", "X670E", "X670", "B650E", "B650", "A620", // AMD AM5
            "X570", "B550", "A520", "X470", "B450", "X370", "B350", // AMD AM4
            "Z890", "B860", // Intel Arrow Lake
            "Z790", "B760", "H770", "H710", // Intel 13/14th
            "Z690", "B660", "H670", "H610", // Intel 12th
            "Z590", "B560", "H570", "H510", // Intel 11th
            "Z490", "B460", "H470", "H410", // Intel 10th
        ];
        
        board.chipset = chipsets.iter()
            .find(|&&c| product_upper.contains(c))
            .unwrap_or(&"Unknown")
            .to_string();

        // 2. Analyze Slots
        let mut ssd_info = SlotInfo::default();
        let mut gpu_info = SlotInfo::default();

        for slot in &slots {
            let des = slot.slot_designation.clone().unwrap_or_default().to_uppercase();
            let is_used = slot.current_usage.unwrap_or(3) == 4; // 4 = In Use

            // GPU Slots (PCIEX16)
            if des.contains("PCIEX16") || des.contains("PCIE_16") || des.contains("PCI-E X16") {
                gpu_info.total += 1;
                if is_used {
                    gpu_info.used += 1;
                }
                gpu_info.details.push(format!("{}: {}", slot.slot_designation.clone().unwrap_or_default(), if is_used { "In Use" } else { "Empty" }));
            }
            
            // SSD Slots (M.2)
            // Note: Win32_SystemSlot often misses M.2 slots on some boards, but it's the best standard way.
            if des.contains("M.2") || des.contains("M2_") {
                ssd_info.total += 1;
                if is_used {
                    ssd_info.used += 1;
                }
                // Try to guess spec from name if possible, otherwise just list
                // e.g. M2_1 (CPU_PCIE5.0)
                ssd_info.details.push(format!("{}: {}", slot.slot_designation.clone().unwrap_or_default(), if is_used { "In Use" } else { "Empty" }));
            }
        }
        
        board.ssd_slots = ssd_info;
        board.gpu_slots = gpu_info;
    }

    Ok(boards)
}
