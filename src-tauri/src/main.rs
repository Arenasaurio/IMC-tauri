// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use rusqlite::{params, Connection, Result};
use std::sync::Mutex;
use tauri::{State, Builder};
// Esta estructura nos servirá para almacenar la conexión a la base de datos
struct DbConnection(Mutex<Connection>);

// Comando para insertar datos y calcular el IMC
#[tauri::command]
fn calcular_y_guardar_imc(
    nombre: String,
    peso: f32,
    talla: f32,
    db: State<DbConnection>,
) -> Result<String, String> {
    if peso <= 0.0 || talla <= 0.0 {
        return Err("Peso y talla deben ser mayores a cero".to_string());
    }

    let imc = peso / (talla * talla);
    let imc_redondeado = (imc * 100.0).round() / 100.0;

    let conn = db.0.lock().map_err(|_| "No se pudo bloquear la conexión a la base de datos".to_string())?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS personas(
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            nombre TEXT NOT NULL,
            peso REAL NOT NULL,
            talla REAL NOT NULL,
            imc REAL NOT NULL
        )",
        [],)
    .map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO personas (nombre, peso, talla, imc) VALUES (?1, ?2, ?3, ?4)",
        params![nombre, peso, talla, imc_redondeado],
    )
    .map_err(|e| e.to_string())?;

    Ok(format!("Tu IMC es: {}", imc_redondeado))
}
fn main() {
    let home_dir = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .expect("No se pudo obtener el directorio home del usuario");

    let app_dir = format!("{}/.tauri_imc", home_dir);
    std::fs::create_dir_all(&app_dir).expect("No se pudo crear el directorio de la aplicación");

    let db_path = format!("{}/imc.db", app_dir);

    println!("Base de datos en: {}", db_path);

    // Crear la conexión a la base de datos
    let conn = Connection::open(&db_path).expect("No se pudo abrir la base de datos");
    

    Builder::default()
        .manage(DbConnection(Mutex::new(conn)))
        .invoke_handler(tauri::generate_handler![calcular_y_guardar_imc])
        .run(tauri::generate_context!())
        .expect("Error al ejecutar la aplicación Tauri");
}
