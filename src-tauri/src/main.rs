#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde_json::json;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn fetch(_set: &str) -> String {
    let hello_world = json!([{
        "name": "world"
    }]);
    
    hello_world.to_string()
}

#[tauri::command]
fn columns(tab: &str) -> String {
    match tab.to_lowercase().as_str() {
        "appointments" => "a,b,c,d".to_string(),
        "patients" => "a,a,c".to_string(),
        _ => "".to_string()
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![fetch, columns])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
