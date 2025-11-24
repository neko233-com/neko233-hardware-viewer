use std::process::Command;

pub fn run_cmd(cmd: &str, args: &[&str]) -> Result<String, String> {
    let output = Command::new(cmd)
        .args(args)
        .output()
        .map_err(|e| e.to_string())?;
        
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

#[tauri::command]
pub fn check_disk_health() -> Result<String, String> {
    // wmic diskdrive get status
    run_cmd("wmic", &["diskdrive", "get", "status"])
}

#[tauri::command]
pub fn check_system_file_integrity() -> Result<String, String> {
    // sfc /verifyonly (scannow requires admin and takes long, verifyonly is safer for check)
    // Note: This usually requires Admin.
    run_cmd("sfc", &["/verifyonly"])
}

#[tauri::command]
pub fn check_dism_health() -> Result<String, String> {
    run_cmd("dism", &["/online", "/cleanup-image", "/checkhealth"])
}

#[tauri::command]
pub fn check_battery_health() -> Result<String, String> {
    // powercfg /batteryreport
    // This generates a file. We might just want to return the path.
    let output = run_cmd("powercfg", &["/batteryreport", "/output", "battery-report.html"])?;
    Ok(format!("Report generated: battery-report.html\n{}", output))
}

#[tauri::command]
pub fn check_network_latency() -> Result<String, String> {
    run_cmd("ping", &["8.8.8.8", "-n", "4"])
}

#[tauri::command]
pub fn check_dns_resolution() -> Result<String, String> {
    run_cmd("nslookup", &["google.com"])
}

#[tauri::command]
pub fn check_activation_status_detailed() -> Result<String, String> {
    run_cmd("cscript", &["//nologo", "C:\\Windows\\System32\\slmgr.vbs", "/xpr"])
}

#[tauri::command]
pub fn check_tpm_status() -> Result<String, String> {
    // wmic /namespace:\\root\cimv2\security\microsofttpm path win32_tpm get IsEnabled_InitialValue
    run_cmd("wmic", &["/namespace:\\\\root\\cimv2\\security\\microsofttpm", "path", "win32_tpm", "get", "IsEnabled_InitialValue"])
}

#[tauri::command]
pub fn check_secure_boot_status() -> Result<String, String> {
    // powershell Confirm-SecureBootUEFI
    let output = Command::new("powershell")
        .args(&["-NoProfile", "-Command", "Confirm-SecureBootUEFI"])
        .output()
        .map_err(|e| e.to_string())?;
    
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Ok("False/Not Supported".to_string())
    }
}

#[tauri::command]
pub fn check_antivirus_status() -> Result<String, String> {
    // wmic /namespace:\\root\SecurityCenter2 path AntivirusProduct get displayName
    run_cmd("wmic", &["/namespace:\\\\root\\SecurityCenter2", "path", "AntivirusProduct", "get", "displayName"])
}
