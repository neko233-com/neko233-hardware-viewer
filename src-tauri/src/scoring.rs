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
    if size_gb >= 900 { // ~1TB
        Score::Excellent
    } else if size_gb >= 450 { // ~500GB
        Score::Good
    } else if size_gb >= 200 {
        Score::Average
    } else {
        Score::Poor
    }
}
