use std::path::Path;

use anyhow::Result;
use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions, Pool, Sqlite};

// データベース接続プールの初期化
pub async fn init_db(db_path: &Path) -> Result<Pool<Sqlite>> {
    // データベースURLを作成
    let path_str = db_path.to_str().unwrap();
    let db_url = format!("sqlite:{}", path_str);

    // データベースが存在しない場合は作成
    if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
        Sqlite::create_database(&db_url).await?;
    }

    // 接続プールを作成
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
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
