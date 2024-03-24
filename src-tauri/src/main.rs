// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use rusqlite::{Connection, Result};


#[tauri::command]
fn read_file(path: String) -> Result<String, String> {
    match std::fs::read_to_string(&path) {
        Ok(content) => Ok(content),
        Err(_) => Err("Failed to read file.".to_string()),
    }
}

struct Barber {
    id: i32,
    name: String
}

fn get_barbers(conn: &Connection) {
    // Подготовка SQL-запроса
    let mut statement = conn.prepare("SELECT id, name FROM barbers")?;

    // Выполнение запроса и получение строк
    let rows = statement.query_map([], |row| {
        Ok(Barber {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    }).unwrap();

    // Преобразование строк в вектор барберов
    // let barbers: Barber = rows.collect();
    // rows.collect();
    println!(rows.collect())
}

fn main() {
  tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![read_file, get_barbers])
      .plugin(tauri_plugin_sql::Builder::default().build())
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}

