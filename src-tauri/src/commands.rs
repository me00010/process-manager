use std::sync::Mutex;

use sysinfo::System;
use tauri::State;

use crate::killer;
use crate::models::{KillResult, ProcessInfo};
use crate::ports;
use crate::process;

pub struct AppState {
    pub sys: Mutex<System>,
}

#[tauri::command]
pub fn list_processes(
    keyword: Option<String>,
    state: State<'_, AppState>,
) -> Result<Vec<ProcessInfo>, String> {
    let mut sys = state.sys.lock().map_err(|e| e.to_string())?;
    let port_map = ports::get_port_map();
    Ok(process::collect_processes(&mut sys, &port_map, keyword.as_deref()))
}

#[tauri::command]
pub fn kill_processes(
    pids: Vec<u32>,
    state: State<'_, AppState>,
) -> Result<Vec<KillResult>, String> {
    let sys = state.sys.lock().map_err(|e| e.to_string())?;
    Ok(pids.iter().map(|&pid| killer::kill_process(&sys, pid)).collect())
}

#[tauri::command]
pub fn is_admin() -> bool {
    #[cfg(target_os = "windows")]
    {
        let out = std::process::Command::new("net")
            .args(["session"])
            .output();
        matches!(out, Ok(o) if o.status.success())
    }
    #[cfg(not(target_os = "windows"))]
    {
        true
    }
}
