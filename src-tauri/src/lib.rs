mod commands;
mod killer;
mod models;
mod ports;
mod process;

use std::sync::Mutex;

use commands::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState {
            sys: Mutex::new(sysinfo::System::new_all()),
            port_cache: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            commands::list_processes,
            commands::kill_processes,
            commands::is_admin
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
