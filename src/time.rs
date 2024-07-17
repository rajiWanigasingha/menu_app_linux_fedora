use chrono::Local;
use tauri::command;

#[command]
pub fn time() -> String {
  let current_time = Local::now();
  current_time.format("%H:%M").to_string()
}

#[command]
pub fn date_today() -> String {
  let current_date = Local::now();
  current_date.format("%A").to_string()
}

#[command]
pub fn am_or_pm() -> String {
  Local::now().format("%p").to_string()
}

#[command]
pub fn date() -> String {
  Local::now().format("%a %b %e %Y").to_string()
}