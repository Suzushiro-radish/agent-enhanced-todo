mod commands;
mod db;
use serde::Deserialize;
use std::io::{Read, Write};
use std::net::TcpListener;

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

/// Structure representing commands received from CLI
#[derive(Deserialize)]
struct CliCommand {
    command: String,
    args: Option<serde_json::Value>,
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
        .show_menu_on_left_click(true)
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

            // Start a TCP server to accept commands from CLI
            let app_handle = app.handle().clone();
            std::thread::spawn(move || {
                let listener = TcpListener::bind("127.0.0.1:45678")
                    .expect("failed to bind TCP port on 127.0.0.1:45678");
                println!("TCP server is listening on 127.0.0.1:45678");

                for stream in listener.incoming() {
                    match stream {
                        Ok(mut stream) => {
                            // Process each connection in a separate thread
                            let app_handle = app_handle.clone();
                            std::thread::spawn(move || {
                                let mut buffer = String::new();
                                if let Err(e) = stream.read_to_string(&mut buffer) {
                                    eprintln!("Failed to read TCP stream: {}", e);
                                    return;
                                }
                                println!("Message from CLI: {}", buffer);
                                let cmd: CliCommand = match serde_json::from_str(&buffer) {
                                    Ok(c) => c,
                                    Err(e) => {
                                        let _ = stream.write_all(
                                            format!("Command parsing failed: {}", e).as_bytes(),
                                        );
                                        return;
                                    }
                                };
                                match cmd.command.as_str() {
                                    "add_todo" => {
                                        // args expects a string of TODO text
                                        let todo = cmd
                                            .args
                                            .as_ref()
                                            .and_then(|v| v.as_str())
                                            .unwrap_or("")
                                            .to_string();
                                        let state = app_handle.state::<AppState>();
                                        // Call by blocking in Tauri's async runtime
                                        let result =
                                            tauri::async_runtime::block_on(add_todo(state, todo));
                                        let response = format!("add_todo result: {:?}", result);
                                        let _ = stream.write_all(response.as_bytes());
                                    }
                                    "get_todos" => {
                                        let state = app_handle.state::<AppState>();
                                        let result =
                                            tauri::async_runtime::block_on(get_todos(state));
                                        let response = format!("get_todos result: {:?}", result);
                                        let _ = stream.write_all(response.as_bytes());
                                    }
                                    // Add other commands as needed
                                    _ => {
                                        let _ = stream.write_all(b"unknown command");
                                    }
                                }
                            });
                        }
                        Err(e) => {
                            eprintln!("TCP connection failed: {}", e);
                        }
                    }
                }
            });

            Ok(())
        })
        // Tauri's IPC commands registration
        .invoke_handler(tauri::generate_handler![
            add_todo,
            get_todos,
            update_todo_status,
            delete_todo
        ])
        .build(tauri::generate_context!())
        .expect("Failed to build Tauri app");

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
