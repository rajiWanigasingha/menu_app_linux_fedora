// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod power;
mod system;
mod time;
mod disk;
mod apps;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![ 
      time::time
      ,time::date_today 
      ,time::am_or_pm 
      ,time::date 
      ,disk::disk_data 
      ,system::ram_data 
      ,system::cup_cors 
      ,system::sys_info 
      ,power::lock
      ,power::sleep
      ,power::shutdown
      ,power::reboot
      ,apps::open_apps 
      ,apps::open_file_explorer 
      ,apps::close
  ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
