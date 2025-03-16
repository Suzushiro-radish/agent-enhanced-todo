use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions, Pool, Sqlite};

// Todoの構造体定義
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Todo {
    id: i64,
    name: String,
    done: bool,
}

// データベース接続プールの初期化
async fn init_db() -> Result<Pool<Sqlite>, sqlx::Error> {
    // データベースURLを作成
    let db_url = "sqlite:todos.db";

    // データベースが存在しない場合は作成
    if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
        Sqlite::create_database(db_url).await?;
    }

    // 接続プールを作成
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await?;

    // テーブル作成のマイグレーションを実行
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            done BOOLEAN NOT NULL DEFAULT 0
        )",
    )
    .execute(&pool)
    .await?;

    Ok(pool)
}

// データベースプールを取得する関数
async fn get_db_pool() -> Pool<Sqlite> {
    init_db().await.expect("Failed to initialize database")
}

// TODOを追加する関数
#[tauri::command]
pub async fn add_todo(name: String) -> Result<(), String> {
    let pool = get_db_pool().await;

    sqlx::query("INSERT INTO todos (name, done) VALUES (?, ?)")
        .bind(name)
        .bind(false)
        .execute(&pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

// すべてのTODOを取得する関数
#[tauri::command]
pub async fn get_todos() -> Result<Vec<Todo>, String> {
    let pool = get_db_pool().await;

    let todos = sqlx::query_as::<_, Todo>("SELECT id, name, done FROM todos ORDER BY id")
        .fetch_all(&pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(todos)
}

// TODOの完了状態を更新する関数
#[tauri::command]
pub async fn update_todo_status(id: i64, done: bool) -> Result<(), String> {
    let pool = get_db_pool().await;

    sqlx::query("UPDATE todos SET done = ? WHERE id = ?")
        .bind(done)
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

// TODOを削除する関数
#[tauri::command]
pub async fn delete_todo(id: i64) -> Result<(), String> {
    let pool = get_db_pool().await;

    sqlx::query("DELETE FROM todos WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
