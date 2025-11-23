use colored::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Score {
    Excellent,
    Good,
    Average,
    Poor,
    Unknown,
}

impl Score {
    pub fn to_colored_string(&self) -> ColoredString {
        match self {
            Score::Excellent => "Excellent".green().bold(),
            Score::Good => "Good".green(),
            Score::Average => "Average".yellow(),
            Score::Poor => "Poor".red(),
            Score::Unknown => "Unknown".white(),
        }
    }
}

pub fn score_cpu(cores: u32, clock_mhz: u32) -> Score {
    if cores >= 8 && clock_mhz >= 3500 {
        Score::Excellent
    } else if cores >= 6 && clock_mhz >= 3000 {
        Score::Good
    } else if cores >= 4 {
        Score::Average
    } else {
        Score::Poor
    }
}

pub fn score_ram(total_capacity_gb: u64, avg_speed_mhz: u32) -> Score {
    if total_capacity_gb >= 32 && avg_speed_mhz >= 3200 {
        Score::Excellent
    } else if total_capacity_gb >= 16 && avg_speed_mhz >= 2666 {
        Score::Good
    } else if total_capacity_gb >= 8 {
        Score::Average
    } else {
        Score::Poor
    }
}

pub fn score_gpu(vram_bytes: u64) -> Score {
    let vram_gb = vram_bytes / 1024 / 1024 / 1024;
    if vram_gb >= 8 {
        Score::Excellent
    } else if vram_gb >= 4 {
        Score::Good
    } else if vram_gb >= 2 {
        Score::Average
    } else {
        Score::Poor
    }
}

pub fn score_disk(size_bytes: u64) -> Score {
    let size_gb = size_bytes / 1024 / 1024 / 1024;
    if size_gb >= 1000 {
        Score::Excellent
    } else if size_gb >= 500 {
        Score::Good
    } else if size_gb >= 250 {
        Score::Average
    } else {
        Score::Poor
    }
}

pub fn calculate_cpu_score_num(cores: u32, clock_mhz: u32) -> u32 {
    // Formula: (Cores * 2) + (Clock / 40)
    // Example: 6 cores, 4000MHz -> 12 + 100 = 112
    // Example: 4 cores, 2500MHz -> 8 + 62 = 70
    let score = (cores as f32 * 2.0) + (clock_mhz as f32 / 40.0);
    score as u32
}

pub fn calculate_ram_score_num(total_gb: u64, avg_speed_mhz: u32) -> u32 {
    // Formula: (GB * 1) + (Speed / 40)
    // Example: 16GB, 3200MHz -> 16 + 80 = 96
    // Example: 8GB, 2133MHz -> 8 + 53 = 61
    let score = (total_gb as f32 * 1.0) + (avg_speed_mhz as f32 / 40.0);
    score as u32
}

pub fn calculate_gpu_score_num(vram_bytes: u64) -> u32 {
    let vram_gb = vram_bytes / 1024 / 1024 / 1024;
    // Formula: VRAM * 8
    // Example: 8GB -> 64
    // Example: 12GB -> 96
    let score = vram_gb as f32 * 8.0;
    score as u32
}

pub fn calculate_disk_score_num(size_bytes: u64) -> u32 {
    let size_gb = size_bytes / 1024 / 1024 / 1024;
    // Formula: Size / 10
    // Example: 1000GB -> 100
    // Example: 500GB -> 50
    let score = size_gb as f32 / 10.0;
    score as u32
}
