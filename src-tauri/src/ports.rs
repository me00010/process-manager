use std::collections::HashMap;

use serde_json::Value;

pub fn get_port_map() -> HashMap<u32, Vec<u16>> {
    #[cfg(target_os = "windows")]
    {
        let mut map = HashMap::new();
        let tcp_script = "Get-NetTCPConnection -State Listen | Select-Object LocalPort,OwningProcess | ConvertTo-Json -Compress";
        let udp_script = "Get-NetUDPEndpoint | Select-Object LocalPort,OwningProcess | ConvertTo-Json -Compress";
        if let Some(json) = run_powershell(tcp_script) {
            parse_pairs(&json, &mut map);
        }
        if let Some(json) = run_powershell(udp_script) {
            parse_pairs(&json, &mut map);
        }
        map
    }
    #[cfg(not(target_os = "windows"))]
    {
        HashMap::new()
    }
}

#[cfg(target_os = "windows")]
fn run_powershell(script: &str) -> Option<Value> {
    let out = std::process::Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", script])
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let text = String::from_utf8_lossy(&out.stdout).to_string();
    serde_json::from_str(text.trim()).ok()
}

fn parse_pairs(value: &Value, map: &mut HashMap<u32, Vec<u16>>) {
    match value {
        Value::Array(items) => {
            for item in items {
                add_pair(item, map);
            }
        }
        Value::Object(_) => add_pair(value, map),
        _ => {}
    }
}

fn add_pair(item: &Value, map: &mut HashMap<u32, Vec<u16>>) {
    let port = item
        .get("LocalPort")
        .and_then(Value::as_u64)
        .map(|v| v as u16);
    let pid = item
        .get("OwningProcess")
        .and_then(Value::as_u64)
        .map(|v| v as u32);
    if let (Some(port), Some(pid)) = (port, pid) {
        if port != 0 {
            map.entry(pid).or_default().push(port);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_array_json() {
        let json: Value = serde_json::from_str(
            r#"[
                {"LocalPort": 8080, "OwningProcess": 1234},
                {"LocalPort": 5000, "OwningProcess": 1234},
                {"LocalPort": 443, "OwningProcess": 5678}
            ]"#,
        )
        .unwrap();
        let mut map = HashMap::new();
        parse_pairs(&json, &mut map);
        let p1234 = map.get(&1234).unwrap();
        assert!(p1234.contains(&8080));
        assert!(p1234.contains(&5000));
        assert_eq!(map.get(&5678).unwrap(), &vec![443u16]);
    }

    #[test]
    fn parses_single_object_json() {
        let json: Value =
            serde_json::from_str(r#"{"LocalPort": 3306, "OwningProcess": 999}"#).unwrap();
        let mut map = HashMap::new();
        parse_pairs(&json, &mut map);
        assert_eq!(map.get(&999).unwrap(), &vec![3306u16]);
    }

    #[test]
    fn ignores_port_zero_and_missing_fields() {
        let json: Value = serde_json::from_str(
            r#"[
                {"LocalPort": 0, "OwningProcess": 1234},
                {"LocalPort": 8080},
                {"OwningProcess": 5678}
            ]"#,
        )
        .unwrap();
        let mut map = HashMap::new();
        parse_pairs(&json, &mut map);
        assert!(map.is_empty());
    }

    #[test]
    fn ignores_invalid_input() {
        let mut map = HashMap::new();
        parse_pairs(&Value::Null, &mut map);
        parse_pairs(&Value::String("boom".into()), &mut map);
        assert!(map.is_empty());
    }
}
