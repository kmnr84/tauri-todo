// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod todo;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(todo::TodoList::new())
        .invoke_handler(tauri::generate_handler![
            todo::add_todo,
            todo::get_todos,
            todo::toggle_todo,
            todo::delete_todo
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
