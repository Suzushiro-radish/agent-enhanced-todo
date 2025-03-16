mod commands;
mod db;
use commands::*;
use db::init_db;
use tauri::Manager;

const DB_PATH: &str = "agent-enhanced-todo.db";

struct AppState {
    pub db_pool: sqlx::Pool<sqlx::Sqlite>,
}

fn setup(app: &tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    use tauri::async_runtime::block_on;

    let db_path = app.path().app_data_dir()?.join(DB_PATH);

    let pool = block_on(init_db(db_path.as_path())).expect("failed to initialize database");
    app.manage(AppState { db_pool: pool });

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            setup(app)?;
            Ok(())
        })
        // コマンドを登録
        .invoke_handler(tauri::generate_handler![
            add_todo,
            get_todos,
            update_todo_status,
            delete_todo
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
