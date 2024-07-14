// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn call(pat: &str) -> Result<Vec<wilo_core::WiloListItem>, String> {
    wilo_core::search(pat).map_err(|err| err.to_string())
}

#[tauri::command]
fn exec(mode: u32, command: &str) -> Result<(), String> {
    wilo_core::execute(mode, command).map_err(|err| err.to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![call, exec])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
