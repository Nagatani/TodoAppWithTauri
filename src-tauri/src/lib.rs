use rusqlite::{params, Connection, Result}; // <--- Added params
use std::fs;
use tauri::Manager;

// Define Todo struct
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")] // Added this line
struct Todo {
    id: i64,
    task: String,
    completed: bool,
    due_date: Option<String>, // This field will be serialized as 'dueDate'
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
            completed BOOLEAN NOT NULL DEFAULT 0,
            due_date TEXT NULL
        )",
        [],
    )?;

    // Determine column_exists within a scope, then use it.
    let mut column_exists_flag = false;
    {
        let mut stmt = conn.prepare("PRAGMA table_info(todos);")?;
        let column_names_iter = stmt.query_map([], |row| row.get::<_, String>("name"))?;
        for name_result in column_names_iter {
            if let Ok(name) = name_result {
                if name == "due_date" {
                    column_exists_flag = true;
                    break;
                }
            }
        }
        // stmt and column_names_iter go out of scope here
    }

    if !column_exists_flag {
        conn.execute("ALTER TABLE todos ADD COLUMN due_date TEXT NULL", [])?;
        println!("'due_date' column successfully added to existing 'todos' table.");
    }

    Ok(conn)
}

#[tauri::command]
fn add_todo(app_handle: tauri::AppHandle, task: String, due_date: Option<String>) -> Result<(), String> {
    println!("[Rust] add_todo called with task: '{}', due_date: {:?}", task, due_date); // Added this line
    let conn = get_db_connection(&app_handle)?;
    conn.execute(
        "INSERT INTO todos (task, completed, due_date) VALUES (?1, 0, ?2)",
        params![task, due_date],
    )
    .map_err(|e| format!("Failed to add todo: {}", e))?;
    Ok(())
}

#[tauri::command]
fn get_todos(app_handle: tauri::AppHandle) -> Result<Vec<Todo>, String> {
    let conn = get_db_connection(&app_handle)?;
    let mut stmt = conn.prepare("SELECT id, task, completed, due_date FROM todos")
        .map_err(|e| format!("Failed to prepare statement: {}", e))?;
    let todo_iter = stmt.query_map([], |row| {
        Ok(Todo {
            id: row.get(0)?,
            task: row.get(1)?,
            completed: row.get(2)?,
            due_date: row.get(3)?, // Added
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

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;
    // Note: tauri::test::mock_app() might require tauri's "test" feature.
    // If AppHandle dependent tests are problematic, focus is on direct DB logic.

    #[test]
    fn test_database_schema() -> std::result::Result<(), String> {
        let conn = Connection::open_in_memory().map_err(|e| e.to_string())?;
        // Manually apply schema similar to initialize_database
        conn.execute(
            "CREATE TABLE IF NOT EXISTS todos (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                task TEXT NOT NULL,
                completed BOOLEAN NOT NULL DEFAULT 0,
                due_date TEXT NULL
            )",
            [],
        ).map_err(|e| e.to_string())?;

        let mut stmt = conn.prepare("PRAGMA table_info(todos);").map_err(|e| e.to_string())?;

        // Iterate over rows to find the 'due_date' column and check its properties
        let mut found_due_date_column = false;
        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>("name")?,
                row.get::<_, String>("type")?,
                row.get::<_, i32>("notnull")?, // notnull is 0 for false, 1 for true
                row.get::<_, Option<String>>("dflt_value")?, // default value
                row.get::<_, i32>("pk")? // primary key flag
            ))
        }).map_err(|e| e.to_string())?;

        for row_result in rows {
            let (name, type_name, notnull_val, _dflt_value, _pk) = row_result.map_err(|e| e.to_string())?;
            if name == "due_date" {
                found_due_date_column = true;
                assert_eq!(type_name, "TEXT", "due_date column type should be TEXT");
                assert_eq!(notnull_val, 0, "due_date column should be nullable (NOT NULL constraint is 0)");
                break;
            }
        }
        assert!(found_due_date_column, "due_date column not found in schema");
        Ok(())
    }

    #[test]
    fn test_add_and_get_todos_with_due_date() -> std::result::Result<(), String> {
        // This test focuses on the SQL logic for adding and retrieving todos with due_dates,
        // using an in-memory database directly. It bypasses AppHandle complexities.
        let conn = Connection::open_in_memory().map_err(|e| format!("In-memory DB error: {}", e))?;

        // Manually create the schema
        conn.execute(
            "CREATE TABLE todos (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                task TEXT NOT NULL,
                completed BOOLEAN NOT NULL DEFAULT 0,
                due_date TEXT NULL
            )",
            [],
        ).map_err(|e| format!("Schema creation error: {}", e))?;

        // Test adding a todo with a due date
        let task1 = "Todo with due date".to_string();
        let due_date1 = Some("2023-12-31T10:00:00".to_string()); // Using a more standard ISO format
        conn.execute(
            "INSERT INTO todos (task, completed, due_date) VALUES (?1, 0, ?2)",
            params![task1, due_date1],
        ).map_err(|e| format!("Insert 1 error: {}", e))?;

        // Test adding a todo without a due date
        let task2 = "Todo without due date".to_string();
        let due_date2: Option<String> = None;
        conn.execute(
            "INSERT INTO todos (task, completed, due_date) VALUES (?1, 0, ?2)",
            params![task2, due_date2],
        ).map_err(|e| format!("Insert 2 error: {}", e))?;

        // Test retrieving todos
        let mut stmt = conn.prepare("SELECT id, task, completed, due_date FROM todos ORDER BY id ASC")
            .map_err(|e| format!("Prepare select error: {}", e))?;

        let todos_iter = stmt.query_map([], |row| {
            Ok(Todo { // Todo struct from the outer scope
                id: row.get(0)?,
                task: row.get(1)?,
                completed: row.get(2)?,
                due_date: row.get(3)?,
            })
        }).map_err(|e| format!("Query map error: {}", e))?;

        let mut results = Vec::new();
        for todo_result in todos_iter {
            results.push(todo_result.map_err(|e| format!("Row processing error: {}", e))?);
        }

        assert_eq!(results.len(), 2, "Should retrieve two todos");

        assert_eq!(results[0].task, task1, "Task 1 name mismatch");
        assert_eq!(results[0].due_date, due_date1, "Task 1 due_date mismatch");

        assert_eq!(results[1].task, task2, "Task 2 name mismatch");
        assert_eq!(results[1].due_date, due_date2, "Task 2 due_date mismatch");

        Ok(())
    }
}
