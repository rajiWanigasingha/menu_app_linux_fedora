use std::path::{Path, PathBuf};
use sysinfo::{
  Disks, System,
};

#[derive(Debug, serde::Serialize)]
pub struct DiskData {
  dir: PathBuf,
  space: u64,
  alailable: u64
}

#[tauri::command]
pub fn disk_data() -> Vec<DiskData> {

  let mut data_array = Vec::new();

  let mut sys = System::new_all();

  sys.refresh_all();

  let disks = Disks::new_with_refreshed_list();

  for disk in &disks {
    if disk.mount_point() == Path::new("/") {
     data_array.push(DiskData { dir: disk.mount_point().to_path_buf() ,space: disk.total_space() ,alailable: disk.available_space()});
    }else if disk.mount_point() == Path::new("/home") {
      data_array.push(DiskData { dir: disk.mount_point().to_path_buf() ,space: disk.total_space() ,alailable: disk.available_space()});
    }

  }

  data_array
}
