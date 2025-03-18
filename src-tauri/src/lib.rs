mod commands;
mod db;
use commands::*;
use db::init_db;
use tauri::{
    menu::{Menu, MenuItem},
    Manager,
};

const DB_PATH: &str = "agent-enhanced-todo.db";

struct AppState {
    pub db_pool: sqlx::Pool<sqlx::Sqlite>,
}

/// Initialize the database connection pool.
fn setup_db(app: &tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    use tauri::async_runtime::block_on;

    let db_path = app.path().app_data_dir()?.join(DB_PATH);

    let pool = block_on(init_db(db_path.as_path())).expect("failed to initialize database");
    app.manage(AppState { db_pool: pool });

    Ok(())
}

fn setup_tray_menu(app: &tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&quit_i])?;

    let _ = tauri::tray::TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .menu_on_left_click(true)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "quit" => {
                println!("quit menu item was clicked");
                app.exit(0);
            }
            _ => {
                println!("menu item {:?} not handled", event.id);
            }
        })
        .build(app)?;

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app = tauri::Builder::default()
        .setup(|app| {
            setup_db(app)?;
            setup_tray_menu(app)?;

            Ok(())
        })
        // コマンドを登録
        .invoke_handler(tauri::generate_handler![
            add_todo,
            get_todos,
            update_todo_status,
            delete_todo
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    app.run(|_, e| {
        if let tauri::RunEvent::ExitRequested { api, code, .. } = e {
            if let Some(code) = code {
                println!("exit requested with code {}", code);
            } else {
                println!("exit requested");
                api.prevent_exit();
            }
        }
    });
}
