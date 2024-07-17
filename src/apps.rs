#[tauri::command]
pub fn close() {
    std::process::exit(0);
}


#[tauri::command]
pub fn open_file_explorer(path : String) {
    #[cfg(target_os = "linux")]
    std::process::Command::new("xdg-open")
        .arg(path)
        .spawn()
        .expect("failed to open file explorer");
    
    std::process::exit(0);
}

#[tauri::command]
pub fn open_apps(app : String) {
  #[cfg(target_os = "linux")]
  {
      println!("Attempting to open: {}", app.trim());
      match std::process::Command::new(app.trim()).spawn() {
          Ok(_) => std::process::exit(0),
          Err(e) => eprintln!("Failed to open {}: {}", app, e),
      }
  }
}