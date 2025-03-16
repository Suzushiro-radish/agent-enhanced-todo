mod commands;
use commands::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
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
