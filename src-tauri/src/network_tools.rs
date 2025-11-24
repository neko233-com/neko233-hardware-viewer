use std::process::Command;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NetworkInterface {
    pub name: String,
    pub description: String,
    pub mac_address: String,
    pub ipv4: Vec<String>,
    pub ipv6: Vec<String>,
    pub gateway: Vec<String>,
    pub dns: Vec<String>,
    pub status: String,
    pub speed: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PingResult {
    pub host: String,
    pub latency_ms: i32, // -1 for timeout/error
    pub status: String,
}

#[tauri::command]
pub fn get_public_ip() -> Result<String, String> {
    let output = Command::new("powershell")
        .args(&["-NoProfile", "-Command", "(Invoke-WebRequest -Uri 'https://api.ipify.org' -UseBasicParsing).Content"])
        .output()
        .map_err(|e| e.to_string())?;
        
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err("Unavailable".to_string())
    }
}

#[tauri::command]
pub fn get_network_interfaces_detailed() -> Result<Vec<NetworkInterface>, String> {
    // Use PowerShell to get detailed info as JSON
    // Force UTF-8 encoding to handle Chinese characters correctly
    // Filter for Up interfaces only to reduce clutter
    let ps_script = r#"
        [Console]::OutputEncoding = [System.Text.Encoding]::UTF8
        $adapters = Get-NetAdapter | Where-Object { $_.Status -eq 'Up' }
        $configs = Get-NetIPConfiguration -ErrorAction SilentlyContinue
        
        $result = @()
        foreach ($adapter in $adapters) {
            $config = $configs | Where-Object { $_.InterfaceIndex -eq $adapter.ifIndex }
            
            $ipv4 = @()
            $ipv6 = @()
            $gateway = @()
            $dns = @()

            if ($config) {
                $ipv4 = $config.IPv4Address.IPAddress
                $ipv6 = $config.IPv6Address.IPAddress
                $gateway = $config.IPv4DefaultGateway.NextHop
                $dns = $config.DNSServer.ServerAddresses
            } else {
                # Fallback to Get-NetIPAddress
                $ip_objs = Get-NetIPAddress -InterfaceIndex $adapter.ifIndex -ErrorAction SilentlyContinue
                if ($ip_objs) {
                     $ipv4 = ($ip_objs | Where-Object { $_.AddressFamily -eq 'IPv4' }).IPAddress
                     $ipv6 = ($ip_objs | Where-Object { $_.AddressFamily -eq 'IPv6' }).IPAddress
                }
            }

            # Ensure arrays and handle nulls
            if ($ipv4 -eq $null) { $ipv4 = @() }
            if ($ipv6 -eq $null) { $ipv6 = @() }
            if ($gateway -eq $null) { $gateway = @() }
            if ($dns -eq $null) { $dns = @() }
            
            $result += [PSCustomObject]@{
                name = $adapter.Name
                description = $adapter.InterfaceDescription
                mac_address = $adapter.MacAddress
                ipv4 = $ipv4
                ipv6 = $ipv6
                gateway = $gateway
                dns = $dns
                status = $adapter.Status
                speed = $adapter.LinkSpeed
            }
        }
        $result | ConvertTo-Json -Depth 2
    "#;

    let output = Command::new("powershell")
        .args(&["-NoProfile", "-Command", ps_script])
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    let json_str = String::from_utf8_lossy(&output.stdout);
    
    let json_str = json_str.trim();
    if json_str.is_empty() {
        return Ok(vec![]);
    }

    // Try parsing as Vec first
    if let Ok(interfaces) = serde_json::from_str::<Vec<NetworkInterface>>(json_str) {
        Ok(interfaces)
    } else if let Ok(interface) = serde_json::from_str::<NetworkInterface>(json_str) {
        Ok(vec![interface])
    } else {
        // Fallback or error logging
        Err(format!("Failed to parse network info: {}", json_str))
    }
}

#[tauri::command]
pub async fn ping_hosts(targets: Vec<String>) -> Result<Vec<PingResult>, String> {
    // Use PowerShell System.Net.NetworkInformation.Ping for reliable results and to avoid parsing localized text
    let target_array_str = targets.iter().map(|t| format!("'{}'", t)).collect::<Vec<_>>().join(",");
    
    let ps_script = format!(r#"
        [Console]::OutputEncoding = [System.Text.Encoding]::UTF8
        $targets = @({})
        $results = @()
        foreach ($t in $targets) {{
            try {{
                $ping = New-Object System.Net.NetworkInformation.Ping
                $reply = $ping.Send($t, 2000)
                if ($reply.Status -eq 'Success') {{
                    $results += [PSCustomObject]@{{ host=$t; latency_ms=$reply.RoundtripTime; status="Online" }}
                }} else {{
                    $results += [PSCustomObject]@{{ host=$t; latency_ms=-1; status=$reply.Status.ToString() }}
                }}
            }} catch {{
                $results += [PSCustomObject]@{{ host=$t; latency_ms=-1; status="Error" }}
            }}
        }}
        $results | ConvertTo-Json
    "#, target_array_str);

    let output = Command::new("powershell")
        .args(&["-NoProfile", "-Command", &ps_script])
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    let json_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if json_str.is_empty() {
        return Ok(vec![]);
    }

    if let Ok(results) = serde_json::from_str::<Vec<PingResult>>(&json_str) {
        Ok(results)
    } else if let Ok(result) = serde_json::from_str::<PingResult>(&json_str) {
        Ok(vec![result])
    } else {
        Err(format!("Failed to parse ping results: {}", json_str))
    }
}
