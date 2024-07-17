use tauri::command;
use sysinfo:: System;

#[derive(Debug, serde::Serialize)]
pub struct RamData {
  total: u64,
  used: u64
}

#[derive(Debug, serde::Serialize)]
pub struct SysInfo {
  name: Option<String>,
  kernal: Option<String>
}


#[command]
pub fn ram_data() -> RamData {
  
  let mut sys = System::new_all();

  sys.refresh_all();

  RamData {
    total: sys.total_memory(),
    used: sys.used_memory()
  }
}

#[command]
pub fn cup_cors() -> std::option::Option<usize> {
  let mut sys = System::new_all();

  sys.refresh_all();

  sys.physical_core_count()
}

#[command]
pub fn sys_info() -> SysInfo {
  SysInfo {
    name : System::name(),
    kernal: System::kernel_version()
  }
}
