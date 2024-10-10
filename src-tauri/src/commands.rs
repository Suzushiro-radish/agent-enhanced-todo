use tauri::{AppHandle, Emitter, State};
use crate::state::{Todo, Todos};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub fn add_todo(name: &str, todos: State<'_, Todos>, app: AppHandle) -> Result<String, String> {
    todos.add(name.to_string());
    app.emit("todo-added", todos.todo_list()).map_err(|e| e.to_string()).unwrap();
    Ok("Todo added!".to_string())
}

#[tauri::command]
pub fn get_todos(todos: State<'_, Todos>) -> Result<Vec<Todo>, String> {
    Ok(todos.todo_list())
}
