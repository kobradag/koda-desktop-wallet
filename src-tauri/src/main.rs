// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

fn main() 
{
    tauri::Builder::default()
    .plugin(tauri_plugin_window_state::Builder::default().build())
    .setup(|app| {
        let window = app.get_window("main").unwrap();
        window.eval("window.location.replace('https://wallet.kobradag.online')");
        Ok(())
      })
      .run(tauri::generate_context!())
      .expect("error running koda wallet");
}
