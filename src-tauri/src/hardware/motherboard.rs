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
    #[serde(skip_deserializing, default)]
    pub ram_slots: SlotInfo,
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
    let wmi = ctx.get_wmi()?;
    let mut boards: Vec<MotherboardInfo> = wmi.raw_query("SELECT * FROM Win32_BaseBoard")?;
    
    // Fetch Slots
    let slots: Vec<SystemSlot> = wmi.raw_query("SELECT SlotDesignation, CurrentUsage, Description FROM Win32_SystemSlot").unwrap_or_default();

    // Fetch RAM Slots Info
    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "PascalCase")]
    struct MemArray {
        memory_devices: Option<u16>,
    }
    let mem_arrays: Vec<MemArray> = wmi.raw_query("SELECT MemoryDevices FROM Win32_PhysicalMemoryArray").unwrap_or_default();
    let total_ram_slots = mem_arrays.first().and_then(|m| m.memory_devices).unwrap_or(0) as u32;
    
    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "PascalCase")]
    struct PhysMem {
        capacity: u64,
    }
    let phys_mems: Vec<PhysMem> = wmi.raw_query("SELECT Capacity FROM Win32_PhysicalMemory").unwrap_or_default();
    let used_ram_slots = phys_mems.len() as u32;

    for board in &mut boards {
        // 1. Extract Chipset
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
            if des.contains("M.2") || des.contains("M2_") {
                ssd_info.total += 1;
                if is_used {
                    ssd_info.used += 1;
                }
                ssd_info.details.push(format!("{}: {}", slot.slot_designation.clone().unwrap_or_default(), if is_used { "In Use" } else { "Empty" }));
            }
        }
        
        board.ssd_slots = ssd_info;
        board.gpu_slots = gpu_info;
        board.ram_slots = SlotInfo {
            total: total_ram_slots,
            used: used_ram_slots,
            details: vec![format!("Used {} of {} slots", used_ram_slots, total_ram_slots)],
        };
    }

    Ok(boards)
}
