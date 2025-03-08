use tauri_plugin_sql::{Migration, MigrationKind};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // データベースのマイグレーション設定
    let migrations = vec![Migration {
        version: 1,
        description: "create initial tables",
        sql: "CREATE TABLE IF NOT EXISTS todos (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL, done BOOLEAN NOT NULL DEFAULT 0)",
        kind: MigrationKind::Up,
    }];

    tauri::Builder::default()
        // SQLプラグインを追加し、マイグレーションを設定
        .plugin(tauri_plugin_sql::Builder::new().add_migrations("sqlite:todos.db", migrations).build())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
