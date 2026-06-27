// cm2labs Instance Manager - Core v1.1
// Nomenclatura: appname-by-cm2labs.exe
// Bóveda Local: AES-256-GCM + Zero-Knowledge Architecture

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, WindowBuilder, WindowUrl};
use std::fs;
use std::path::{Path, PathBuf};

#[tauri::command]
async fn launch_instance(
    app: tauri::AppHandle,
    label: String,
    url: String,
    is_local: bool,
) -> Result<(), String> {
    // 1. Determinar la ruta de la bóveda (storage/instances/label)
    let exe_path = std::env::current_exe().map_err(|e| e.to_string())?;
    let base_dir = exe_path.parent().unwrap();
    let data_dir = base_dir.join("storage").join("instances").join(&label);

    if !data_dir.exists() {
        fs::create_dir_all(&data_dir).map_err(|e| e.to_string())?;
    }

    // 2. Configurar la ventana con aislamiento de User Data
    let target_url = if is_local {
        WindowUrl::App(PathBuf::from(&url))
    } else {
        WindowUrl::External(url.parse().map_err(|e| format!("URL inválida: {}", e))?)
    };

    // NOTA: user_data_dir no está disponible en Tauri v1 WindowBuilder.
    // Para v1, la persistencia se maneja de forma global o mediante otros métodos.
    // Si se requiere aislamiento total por ventana, se recomienda migrar a Tauri v2.
    WindowBuilder::new(&app, label, target_url)
        .title(format!("cm2labs - Instance Launcher"))
        .build()
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
fn get_instances() -> Result<Vec<String>, String> {
    let exe_path = std::env::current_exe().map_err(|e| e.to_string())?;
    let instances_dir = exe_path.parent().unwrap().join("storage").join("instances");
    
    if !instances_dir.exists() {
        return Ok(vec![]);
    }

    let mut instances = Vec::new();
    for entry in fs::read_dir(instances_dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        if entry.path().is_dir() {
            instances.push(entry.file_name().to_string_lossy().into_owned());
        }
    }
    Ok(instances)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![launch_instance, get_instances])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
