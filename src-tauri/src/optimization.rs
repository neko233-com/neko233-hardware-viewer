use std::process::Command;
use winreg::enums::*;
use winreg::RegKey;
use std::path::Path;

pub fn run_powershell(cmd: &str) -> Result<String, String> {
    let output = Command::new("powershell")
        .args(&["-NoProfile", "-Command", cmd])
        .output()
        .map_err(|e| e.to_string())?;
        
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

pub fn set_reg_value(key_path: &str, value_name: &str, value: u32) -> Result<(), String> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    
    // Try HKCU first, then HKLM if path starts with HKLM (simplified logic)
    let (root, subkey) = if key_path.starts_with("HKLM") {
        (hklm, key_path.replace("HKLM\\", ""))
    } else {
        (hkcu, key_path.replace("HKCU\\", ""))
    };

    let (key, _) = root.create_subkey(&subkey).map_err(|e| e.to_string())?;
    key.set_value(value_name, &value).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn apply_optimization(id: String, enable: bool) -> Result<String, String> {
    match id.as_str() {
        "telemetry" => {
            set_reg_value("HKLM\\SOFTWARE\\Policies\\Microsoft\\Windows\\DataCollection", "AllowTelemetry", if enable { 0 } else { 1 })?;
            run_powershell(if enable { "Stop-Service DiagTrack -Force; Set-Service DiagTrack -StartupType Disabled" } else { "Set-Service DiagTrack -StartupType Automatic; Start-Service DiagTrack" })?;
            Ok("Telemetry settings updated".to_string())
        },
        "windows_update" => {
            run_powershell(if enable { "Stop-Service wuauserv -Force; Set-Service wuauserv -StartupType Disabled" } else { "Set-Service wuauserv -StartupType Manual" })?;
            Ok("Windows Update service updated".to_string())
        },
        "hibernation" => {
            run_powershell(if enable { "powercfg -h off" } else { "powercfg -h on" })?;
            Ok("Hibernation settings updated".to_string())
        },
        "game_dvr" => {
            let val = if enable { 0 } else { 1 };
            set_reg_value("HKCU\\System\\GameConfigStore", "GameDVR_Enabled", val)?;
            set_reg_value("HKLM\\SOFTWARE\\Policies\\Microsoft\\Windows\\GameDVR", "AllowGameDVR", val)?;
            Ok("Game DVR settings updated".to_string())
        },
        "sticky_keys" => {
            // 506 = Off, 510 = On (Default) roughly
            let val = if enable { 506 } else { 510 };
            set_reg_value("HKCU\\Control Panel\\Accessibility\\StickyKeys", "Flags", val)?;
            Ok("Sticky Keys settings updated".to_string())
        },
        "mouse_acceleration" => {
            let val = if enable { 0 } else { 1 }; // 0 = Off, 1 = On (Standard)
            set_reg_value("HKCU\\Control Panel\\Mouse", "MouseSpeed", val)?;
            set_reg_value("HKCU\\Control Panel\\Mouse", "MouseThreshold1", 0)?;
            set_reg_value("HKCU\\Control Panel\\Mouse", "MouseThreshold2", 0)?;
            Ok("Mouse Acceleration settings updated".to_string())
        },
        "lock_screen" => {
            set_reg_value("HKLM\\SOFTWARE\\Policies\\Microsoft\\Windows\\Personalization", "NoLockScreen", if enable { 1 } else { 0 })?;
            Ok("Lock Screen settings updated".to_string())
        },
        "bing_search" => {
            set_reg_value("HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Search", "BingSearchEnabled", if enable { 0 } else { 1 })?;
            Ok("Bing Search settings updated".to_string())
        },
        "aero_shake" => {
            set_reg_value("HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Advanced", "DisallowShaking", if enable { 1 } else { 0 })?;
            Ok("Aero Shake settings updated".to_string())
        },
        "timeline" => {
            set_reg_value("HKLM\\SOFTWARE\\Policies\\Microsoft\\Windows\\System", "EnableActivityFeed", if enable { 0 } else { 1 })?;
            Ok("Timeline settings updated".to_string())
        },
        "cortana" => {
            set_reg_value("HKLM\\SOFTWARE\\Policies\\Microsoft\\Windows\\Windows Search", "AllowCortana", if enable { 0 } else { 1 })?;
            Ok("Cortana settings updated".to_string())
        },
        "location_tracking" => {
            set_reg_value("HKLM\\SOFTWARE\\Policies\\Microsoft\\Windows\\LocationAndSensors", "DisableLocation", if enable { 1 } else { 0 })?;
            Ok("Location Tracking settings updated".to_string())
        },
        "advertising_id" => {
            set_reg_value("HKLM\\SOFTWARE\\Policies\\Microsoft\\Windows\\AdvertisingInfo", "DisabledByGroupPolicy", if enable { 1 } else { 0 })?;
            Ok("Advertising ID settings updated".to_string())
        },
        "feedback_diagnostics" => {
            set_reg_value("HKLM\\SOFTWARE\\Policies\\Microsoft\\Windows\\DataCollection", "AllowTelemetry", if enable { 0 } else { 3 })?;
            Ok("Feedback & Diagnostics settings updated".to_string())
        },
        "suggested_apps" => {
            set_reg_value("HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\ContentDeliveryManager", "SystemPaneSuggestionsEnabled", if enable { 0 } else { 1 })?;
            Ok("Suggested Apps settings updated".to_string())
        },
        "meet_now" => {
            set_reg_value("HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Policies\\Explorer", "HideSCAMeetNow", if enable { 1 } else { 0 })?;
            Ok("Meet Now icon settings updated".to_string())
        },
        "news_interests" => {
            set_reg_value("HKLM\\SOFTWARE\\Policies\\Microsoft\\Windows\\Windows Feeds", "EnableFeeds", if enable { 0 } else { 1 })?;
            Ok("News and Interests settings updated".to_string())
        },
        "wifi_sense" => {
            set_reg_value("HKLM\\SOFTWARE\\Microsoft\\WcmSvc\\wifinetworkmanager\\config", "AutoConnectAllowedOEM", if enable { 0 } else { 1 })?;
            Ok("Wifi Sense settings updated".to_string())
        },
        "delivery_optimization" => {
            set_reg_value("HKLM\\SOFTWARE\\Policies\\Microsoft\\Windows\\DeliveryOptimization", "DODownloadMode", if enable { 0 } else { 1 })?;
            Ok("Delivery Optimization settings updated".to_string())
        },
        "this_pc_desktop" => {
            // {20D04FE0-3AEA-1069-A2D8-08002B30309D}
            // This is complex, usually requires HideDesktopIcons\NewStartPanel
            let val = if enable { 0 } else { 1 };
            set_reg_value("HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\HideDesktopIcons\\NewStartPanel", "{20D04FE0-3AEA-1069-A2D8-08002B30309D}", val)?;
            run_powershell("taskkill /f /im explorer.exe; start explorer.exe")?;
            Ok("This PC icon settings updated".to_string())
        },
        "control_panel_desktop" => {
            // {5399E694-6CE5-4D6C-8FCE-1D8870FDCBA0}
            let val = if enable { 0 } else { 1 };
            set_reg_value("HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\HideDesktopIcons\\NewStartPanel", "{5399E694-6CE5-4D6C-8FCE-1D8870FDCBA0}", val)?;
            run_powershell("taskkill /f /im explorer.exe; start explorer.exe")?;
            Ok("Control Panel icon settings updated".to_string())
        },
        "dark_mode" => {
            let val = if enable { 0 } else { 1 };
            set_reg_value("HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize", "AppsUseLightTheme", val)?;
            set_reg_value("HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize", "SystemUseLightTheme", val)?;
            Ok("Dark Mode settings updated".to_string())
        },
        "transparency" => {
            set_reg_value("HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize", "EnableTransparency", if enable { 0 } else { 1 })?;
            Ok("Transparency settings updated".to_string())
        },
        "high_perf_plan" => {
            if enable {
                run_powershell("powercfg -setactive 8c5e7fda-e8bf-4a96-9a85-a6e23a8c635c")?;
                Ok("High Performance Plan activated".to_string())
            } else {
                run_powershell("powercfg -setactive 381b4222-f694-41f0-9685-ff5bb260df2e")?; // Balanced
                Ok("Balanced Plan activated".to_string())
            }
        },
        "fast_startup" => {
            set_reg_value("HKLM\\SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Power", "HiberbootEnabled", if enable { 0 } else { 1 })?;
            Ok("Fast Startup settings updated".to_string())
        },
        "uac_dimming" => {
            set_reg_value("HKLM\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\System", "PromptOnSecureDesktop", if enable { 0 } else { 1 })?;
            Ok("UAC Dimming settings updated".to_string())
        },
        "printer_spooler" => {
            run_powershell(if enable { "Stop-Service Spooler -Force; Set-Service Spooler -StartupType Disabled" } else { "Set-Service Spooler -StartupType Automatic; Start-Service Spooler" })?;
            Ok("Printer Spooler settings updated".to_string())
        },
        "fax_service" => {
            run_powershell(if enable { "Stop-Service Fax -Force; Set-Service Fax -StartupType Disabled" } else { "Set-Service Fax -StartupType Manual" })?;
            Ok("Fax Service settings updated".to_string())
        },
        "xps_services" => {
            run_powershell(if enable { "Stop-Service xpsservices -Force; Set-Service xpsservices -StartupType Disabled" } else { "Set-Service xpsservices -StartupType Manual" })?;
            Ok("XPS Services settings updated".to_string())
        },
        "error_reporting" => {
            set_reg_value("HKLM\\SOFTWARE\\Microsoft\\Windows\\Windows Error Reporting", "Disabled", if enable { 1 } else { 0 })?;
            Ok("Error Reporting settings updated".to_string())
        },
        _ => Err(format!("Unknown optimization ID: {}", id))
    }
}

#[tauri::command]
pub fn disable_telemetry() -> Result<String, String> {
    apply_optimization("telemetry".to_string(), true)
}

#[tauri::command]
pub fn disable_windows_update() -> Result<String, String> {
    apply_optimization("windows_update".to_string(), true)
}

#[tauri::command]
pub fn disable_hibernation() -> Result<String, String> {
    apply_optimization("hibernation".to_string(), true)
}

#[tauri::command]
pub fn disable_game_dvr() -> Result<String, String> {
    apply_optimization("game_dvr".to_string(), true)
}

#[tauri::command]
pub fn disable_sticky_keys() -> Result<String, String> {
    apply_optimization("sticky_keys".to_string(), true)
}

#[tauri::command]
pub fn disable_mouse_acceleration() -> Result<String, String> {
    apply_optimization("mouse_acceleration".to_string(), true)
}

#[tauri::command]
pub fn disable_lock_screen() -> Result<String, String> {
    apply_optimization("lock_screen".to_string(), true)
}

#[tauri::command]
pub fn disable_bing_search() -> Result<String, String> {
    apply_optimization("bing_search".to_string(), true)
}

#[tauri::command]
pub fn disable_aero_shake() -> Result<String, String> {
    apply_optimization("aero_shake".to_string(), true)
}

#[tauri::command]
pub fn disable_timeline() -> Result<String, String> {
    apply_optimization("timeline".to_string(), true)
}

#[tauri::command]
pub fn disable_feedback_hub() -> Result<String, String> {
    set_reg_value("HKLM\\SOFTWARE\\Policies\\Microsoft\\Windows\\DataCollection", "DoNotShowFeedbackNotifications", 1)?;
    Ok("Feedback Hub notifications disabled".to_string())
}

#[tauri::command]
pub fn disable_location_tracking() -> Result<String, String> {
    set_reg_value("HKLM\\SOFTWARE\\Policies\\Microsoft\\Windows\\LocationAndSensors", "DisableLocation", 1)?;
    Ok("Location Tracking disabled".to_string())
}

#[tauri::command]
pub fn disable_advertising_id() -> Result<String, String> {
    set_reg_value("HKLM\\SOFTWARE\\Policies\\Microsoft\\Windows\\AdvertisingInfo", "DisabledByGroupPolicy", 1)?;
    Ok("Advertising ID disabled".to_string())
}

#[tauri::command]
pub fn disable_error_reporting() -> Result<String, String> {
    set_reg_value("HKLM\\SOFTWARE\\Microsoft\\Windows\\Windows Error Reporting", "Disabled", 1)?;
    Ok("Error Reporting disabled".to_string())
}

#[tauri::command]
pub fn disable_compatibility_assistant() -> Result<String, String> {
    run_powershell("Stop-Service PcaSvc -Force; Set-Service PcaSvc -StartupType Disabled")?;
    Ok("Program Compatibility Assistant disabled".to_string())
}

#[tauri::command]
pub fn disable_print_spooler() -> Result<String, String> {
    run_powershell("Stop-Service Spooler -Force; Set-Service Spooler -StartupType Disabled")?;
    Ok("Print Spooler disabled".to_string())
}

#[tauri::command]
pub fn disable_remote_registry() -> Result<String, String> {
    run_powershell("Stop-Service RemoteRegistry -Force; Set-Service RemoteRegistry -StartupType Disabled")?;
    Ok("Remote Registry disabled".to_string())
}

#[tauri::command]
pub fn set_dns_cloudflare() -> Result<String, String> {
    run_powershell("Get-NetAdapter | Where-Object {$_.Status -eq 'Up'} | Set-DnsClientServerAddress -ServerAddresses ('1.1.1.1', '1.0.0.1')")?;
    Ok("DNS set to Cloudflare (1.1.1.1)".to_string())
}

#[tauri::command]
pub fn set_dns_google() -> Result<String, String> {
    run_powershell("Get-NetAdapter | Where-Object {$_.Status -eq 'Up'} | Set-DnsClientServerAddress -ServerAddresses ('8.8.8.8', '8.8.4.4')")?;
    Ok("DNS set to Google (8.8.8.8)".to_string())
}

#[tauri::command]
pub fn clear_temp_files() -> Result<String, String> {
    run_powershell("Remove-Item -Path $env:TEMP\\* -Recurse -Force -ErrorAction SilentlyContinue; Remove-Item -Path 'C:\\Windows\\Temp\\*' -Recurse -Force -ErrorAction SilentlyContinue")?;
    Ok("Temporary files cleared".to_string())
}

#[tauri::command]
pub fn clear_dns_cache() -> Result<String, String> {
    run_powershell("Clear-DnsClientCache")?;
    Ok("DNS Cache cleared".to_string())
}

#[tauri::command]
pub fn reset_network_stack() -> Result<String, String> {
    run_powershell("netsh winsock reset; netsh int ip reset")?;
    Ok("Network stack reset (Winsock/IP)".to_string())
}
