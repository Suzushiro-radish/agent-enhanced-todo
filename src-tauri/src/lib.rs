use state::Todos;

mod commands;
mod state;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Todos::new())
        .invoke_handler(tauri::generate_handler![commands::greet, commands::add_todo, commands::get_todos])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
