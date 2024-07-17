#[tauri::command]

pub fn lock() {
    #[cfg(target_os = "linux")]
    std::process::Command::new("hyprlock")
        .spawn()
        .expect("Opration faild");
    
    std::process::exit(0);
}

#[tauri::command]
pub fn sleep() {
    #[cfg(target_os = "linux")]
    std::process::Command::new("systemctl")
        .arg("suspend")
        .spawn()
        .expect("Opration faild");

    std::process::Command::new("hyprlock")
        .spawn()
        .expect("Opration faild");
    
    std::process::exit(0);
}

#[tauri::command]
pub fn shutdown() {
    #[cfg(target_os = "linux")]
    std::process::Command::new("systemctl")
        .arg("poweroff")
        .spawn()
        .expect("Opration faild");
    
    std::process::exit(0);
}

#[tauri::command]
pub fn reboot() {
    #[cfg(target_os = "linux")]
    std::process::Command::new("systemctl")
        .arg("reboot")
        .spawn()
        .expect("Opration faild");
    
    std::process::exit(0);
}