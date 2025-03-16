use anyhow::Result;
use serde::{Deserialize, Serialize};
use tauri::State;

use crate::AppState;

// Todoの構造体定義
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Todo {
    id: i64,
    name: String,
    done: bool,
}

// TODOを追加する関数
#[tauri::command]
pub async fn add_todo(state: State<'_, AppState>, name: String) -> Result<(), String> {
    sqlx::query("INSERT INTO todos (name, done) VALUES (?, ?)")
        .bind(name)
        .bind(false)
        .execute(&state.db_pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

// すべてのTODOを取得する関数
#[tauri::command]
pub async fn get_todos(state: State<'_, AppState>) -> Result<Vec<Todo>, String> {
    let todos = sqlx::query_as::<_, Todo>("SELECT id, name, done FROM todos ORDER BY id")
        .fetch_all(&state.db_pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(todos)
}

// TODOの完了状態を更新する関数
#[tauri::command]
pub async fn update_todo_status(
    state: State<'_, AppState>,
    id: i64,
    done: bool,
) -> Result<(), String> {
    sqlx::query("UPDATE todos SET done = ? WHERE id = ?")
        .bind(done)
        .bind(id)
        .execute(&state.db_pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

// TODOを削除する関数
#[tauri::command]
pub async fn delete_todo(state: State<'_, AppState>, id: i64) -> Result<(), String> {
    sqlx::query("DELETE FROM todos WHERE id = ?")
        .bind(id)
        .execute(&state.db_pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
