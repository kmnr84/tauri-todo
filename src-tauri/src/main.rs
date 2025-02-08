// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod domain;
mod todo_usecase;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(todo_usecase::TodoList::new())
        .invoke_handler(tauri::generate_handler![
            todo_usecase::add_todo,
            todo_usecase::get_todos,
            todo_usecase::toggle_todo,
            todo_usecase::delete_todo
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
