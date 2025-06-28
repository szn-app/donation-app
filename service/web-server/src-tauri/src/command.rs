// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
pub fn greet(name: &str) -> String {
    format!(
        "Rust code executed with {} parameter (using Tauri frontend execution model.)",
        name
    )
}
