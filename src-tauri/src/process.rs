use std::collections::HashMap;

use sysinfo::System;

use crate::killer::is_protected;
use crate::models::ProcessInfo;

pub fn collect_processes(
    sys: &mut System,
    port_map: &HashMap<u32, Vec<u16>>,
    keyword: Option<&str>,
) -> Vec<ProcessInfo> {
    sys.refresh_processes();
    sys.refresh_cpu_usage();

    let mut result: Vec<ProcessInfo> = sys
        .processes()
        .iter()
        .map(|(pid, proc)| {
            let name = proc.name().to_string();
            let pid_u32 = pid.as_u32();
            ProcessInfo {
                pid: pid_u32,
                ports: port_map.get(&pid_u32).cloned().unwrap_or_default(),
                name: name.clone(),
                cpu: proc.cpu_usage(),
                memory: proc.memory(),
                path: proc.exe().map(|p| p.to_string_lossy().to_string()),
                protected: is_protected(&name),
            }
        })
        .collect();

    filter_process_list(&mut result, keyword);

    result.sort_by(|a, b| a.pid.cmp(&b.pid));
    result
}

fn filter_process_list(list: &mut Vec<ProcessInfo>, keyword: Option<&str>) {
    if let Some(kw) = keyword {
        let kw = kw.trim();
        if !kw.is_empty() {
            let kw_lower = kw.to_lowercase();
            list.retain(|info| matches_keyword(info, kw, &kw_lower));
        }
    }
}

fn matches_keyword(info: &ProcessInfo, keyword: &str, keyword_lower: &str) -> bool {
    if info.name.to_lowercase().contains(keyword_lower) {
        return true;
    }
    if let Ok(pid) = keyword.parse::<u32>() {
        if info.pid == pid {
            return true;
        }
        if info.ports.iter().any(|&p| p as u32 == pid) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample(pid: u32, name: &str, ports: Vec<u16>) -> ProcessInfo {
        ProcessInfo {
            pid,
            name: name.to_string(),
            cpu: 0.0,
            memory: 0,
            path: None,
            ports,
            protected: false,
        }
    }

    #[test]
    fn matches_by_pid() {
        let info = sample(1234, "node.exe", vec![]);
        assert!(matches_keyword(&info, "1234", "1234"));
        assert!(!matches_keyword(&info, "1235", "1235"));
    }

    #[test]
    fn matches_by_port() {
        let info = sample(777, "python.exe", vec![8080]);
        assert!(matches_keyword(&info, "8080", "8080"));
        assert!(!matches_keyword(&info, "8081", "8081"));
    }

    #[test]
    fn matches_by_name_case_insensitive() {
        let info = sample(1, "MyApp.EXE", vec![]);
        assert!(matches_keyword(&info, "myapp", "myapp"));
        assert!(matches_keyword(&info, "APP", "app"));
        assert!(!matches_keyword(&info, "xyz", "xyz"));
    }

    #[test]
    fn empty_or_whitespace_keyword_keeps_all() {
        let mut list = vec![
            sample(1, "node.exe", vec![]),
            sample(2, "python.exe", vec![8080]),
        ];
        filter_process_list(&mut list, None);
        assert_eq!(list.len(), 2);
        filter_process_list(&mut list, Some("   "));
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn keyword_filters_list() {
        let mut list = vec![
            sample(1, "node.exe", vec![]),
            sample(2, "python.exe", vec![8080]),
        ];
        filter_process_list(&mut list, Some("python"));
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].pid, 2);
    }
}
