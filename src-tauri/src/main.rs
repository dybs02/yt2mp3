// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod download;
mod utils;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      download::download,
      utils::get_image_b64
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
