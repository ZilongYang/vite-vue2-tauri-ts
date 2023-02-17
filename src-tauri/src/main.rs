#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

fn main() {

  let context = tauri::generate_context!();

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      greet,
      greet1
    ])
    .run(context)
    .expect("error while running tauri application");
}


#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}", name)
}
#[tauri::command]
fn greet1(name: &str) -> String {
    format!("Hello, {}", name)
}