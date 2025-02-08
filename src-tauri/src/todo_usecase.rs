use tauri::State;
use std::sync::Mutex;

use crate::domain::todo::Todo;

#[derive(Default)]
pub struct TodoList(Mutex<Vec<Todo>>);

impl TodoList {
    pub fn new() -> Self {
        Self(Mutex::new(Vec::new()))
    }
}

#[tauri::command]
pub fn add_todo(state: State<'_, TodoList>, title: String) {
    // println!("add_todo: title: {}", title);
    let mut todos = state.0.lock().unwrap();
    todos.push(Todo::new(title).unwrap());
}

#[tauri::command]
pub fn get_todos(state: State<'_, TodoList>) -> Vec<Todo> {
    state.0.lock().unwrap().clone()
}

#[tauri::command]
pub fn toggle_todo(state: State<'_, TodoList>, index: usize, completed: bool) {
    // println!("toggle_todo: index: {}, completed: {}", index, completed);
    let mut todos = state.0.lock().unwrap();
    if let Some(todo) = todos.get_mut(index) {
        if completed {
            todo.complete();
        } else {
            todo.uncomplete();
        }
    }
}

#[tauri::command]
pub fn delete_todo(state: State<'_, TodoList>, index: usize) {
    let mut todos = state.0.lock().unwrap();
    if index < todos.len() {
        todos.remove(index);
    }
}