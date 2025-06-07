use rusqlite::{params, Connection, Result}; // <--- Added params
use std::fs;
use tauri::Manager;

// Define Todo struct
#[derive(serde::Serialize)]
struct Todo {
    id: i64,
    task: String,
    completed: bool,
}

// Helper function to get database connection
fn get_db_connection(app_handle: &impl tauri::Manager<tauri::Wry>) -> Result<Connection, String> {
    let app_data_path = app_handle.path().app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?
        .join("db");

    let db_path = app_data_path.join("todo.db");
    Connection::open(db_path).map_err(|e| format!("Failed to open database: {}", e))
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn initialize_database(app_handle: &impl Manager<tauri::Wry>) -> Result<Connection, rusqlite::Error> {
    let app_data_path = app_handle.path().app_data_dir()
        .expect("Failed to get app data directory")
        .join("db");

    if !app_data_path.exists() {
        fs::create_dir_all(&app_data_path).expect("Failed to create app data directory");
    }

    let db_path = app_data_path.join("todo.db");
    let conn = Connection::open(db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            task TEXT NOT NULL,
            completed BOOLEAN NOT NULL DEFAULT 0
        )",
        [],
    )?;
    Ok(conn)
}

#[tauri::command]
fn add_todo(app_handle: tauri::AppHandle, task: String) -> Result<(), String> {
    let conn = get_db_connection(&app_handle)?;
    conn.execute(
        "INSERT INTO todos (task, completed) VALUES (?1, 0)",
        params![task],
    )
    .map_err(|e| format!("Failed to add todo: {}", e))?;
    Ok(())
}

#[tauri::command]
fn get_todos(app_handle: tauri::AppHandle) -> Result<Vec<Todo>, String> {
    let conn = get_db_connection(&app_handle)?;
    let mut stmt = conn.prepare("SELECT id, task, completed FROM todos")
        .map_err(|e| format!("Failed to prepare statement: {}", e))?;
    let todo_iter = stmt.query_map([], |row| {
        Ok(Todo {
            id: row.get(0)?,
            task: row.get(1)?,
            completed: row.get(2)?,
        })
    })
    .map_err(|e| format!("Failed to query todos: {}", e))?;

    let mut todos = Vec::new();
    for todo in todo_iter {
        todos.push(todo.map_err(|e| format!("Failed to process todo row: {}", e))?);
    }
    Ok(todos)
}

#[tauri::command]
fn update_todo_status(app_handle: tauri::AppHandle, id: i64, completed: bool) -> Result<(), String> {
    let conn = get_db_connection(&app_handle)?;
    conn.execute(
        "UPDATE todos SET completed = ?1 WHERE id = ?2",
        params![completed, id],
    )
    .map_err(|e| format!("Failed to update todo: {}", e))?;
    Ok(())
}

#[tauri::command]
fn delete_todo(app_handle: tauri::AppHandle, id: i64) -> Result<(), String> {
    let conn = get_db_connection(&app_handle)?;
    conn.execute("DELETE FROM todos WHERE id = ?1", params![id])
        .map_err(|e| format!("Failed to delete todo: {}", e))?;
    Ok(())
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            match initialize_database(app) {
                Ok(_conn) => {
                    println!("Database initialized successfully during setup.");
                }
                Err(e) => {
                    eprintln!("Failed to initialize database: {:?}", e);
                }
            }
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            add_todo,
            get_todos,
            update_todo_status,
            delete_todo
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
