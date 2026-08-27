use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use sysinfo::System;
use tauri::State;

use crate::killer;
use crate::models::{KillResult, ProcessInfo};
use crate::ports;
use crate::process;

const PORT_MAP_TTL: Duration = Duration::from_secs(5);

pub(crate) struct PortMapCache {
    data: HashMap<u32, Vec<u16>>,
    updated_at: Instant,
}
pub struct AppState {
    pub sys: Mutex<System>,
    pub port_cache: Mutex<Option<PortMapCache>>,
}

#[tauri::command]
pub fn list_processes(
    keyword: Option<String>,
    state: State<'_, AppState>,
) -> Result<Vec<ProcessInfo>, String> {
    let mut sys = state.sys.lock().map_err(|e| e.to_string())?;
    let port_map = {
        let mut cache = state.port_cache.lock().map_err(|e| e.to_string())?;
        let fresh = matches!(&*cache, Some(c) if c.updated_at.elapsed() < PORT_MAP_TTL);
        if fresh {
            cache.as_ref().unwrap().data.clone()
        } else {
            let map = ports::get_port_map();
            *cache = Some(PortMapCache {
                data: map.clone(),
                updated_at: Instant::now(),
            });
            map
        }
    };
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
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        let out = std::process::Command::new("net")
            .args(["session"])
            .creation_flags(CREATE_NO_WINDOW)
            .output();
        matches!(out, Ok(o) if o.status.success())
    }
    #[cfg(not(target_os = "windows"))]
    {
        true
    }
}
