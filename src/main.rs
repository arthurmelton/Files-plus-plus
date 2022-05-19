#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod files;
use files::*;

fn main() {
  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![get_files, home, remove_hidden, remove_folders, remove_files, is_folder, open])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
