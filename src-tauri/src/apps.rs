use winreg::enums::*;
use winreg::RegKey;
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppInfo {
    pub name: String,
    pub version: String,
    pub publisher: String,
    pub uninstall_string: String,
    pub install_date: String,
}

#[tauri::command]
pub fn get_installed_apps() -> Result<Vec<AppInfo>, String> {
    let mut apps = Vec::new();
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    
    let paths = vec![
        "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall",
        "SOFTWARE\\WOW6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall",
    ];

    for path in paths {
        if let Ok(key) = hklm.open_subkey(path) {
            collect_apps_from_key(&key, &mut apps);
        }
        if let Ok(key) = hkcu.open_subkey(path) {
            collect_apps_from_key(&key, &mut apps);
        }
    }

    // Sort by name
    apps.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(apps)
}

fn collect_apps_from_key(key: &RegKey, apps: &mut Vec<AppInfo>) {
    for name in key.enum_keys().map(|x| x.unwrap_or_default()) {
        if let Ok(subkey) = key.open_subkey(&name) {
            let display_name: String = subkey.get_value("DisplayName").unwrap_or_default();
            if display_name.is_empty() { continue; }
            
            let uninstall_string: String = subkey.get_value("UninstallString").unwrap_or_default();
            if uninstall_string.is_empty() { continue; }

            let display_version: String = subkey.get_value("DisplayVersion").unwrap_or_default();
            let publisher: String = subkey.get_value("Publisher").unwrap_or_default();
            let install_date: String = subkey.get_value("InstallDate").unwrap_or_default();

            apps.push(AppInfo {
                name: display_name,
                version: display_version,
                publisher,
                uninstall_string,
                install_date,
            });
        }
    }
}

#[tauri::command]
pub fn uninstall_app(uninstall_string: String) -> Result<String, String> {
    // This is dangerous if we just run it blindly, but that's what the user wants.
    // We should try to parse it.
    // Usually it's "MsiExec.exe /I{GUID}" or "C:\Program Files\...\uninstall.exe"
    
    // Simple approach: Run it in cmd
    let output = Command::new("cmd")
        .args(&["/C", &uninstall_string])
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok("Uninstaller launched".to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}
