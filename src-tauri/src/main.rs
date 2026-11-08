#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs, path::PathBuf, sync::Mutex};
use tauri::{Manager, State};

struct SaveDirectory {
    path: PathBuf,
    write_lock: Mutex<()>,
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let directory = app.path().app_data_dir()?.join("saves");
            fs::create_dir_all(&directory)?;
            app.manage(SaveDirectory {
                path: directory,
                write_lock: Mutex::new(()),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            set_game_fullscreen,
            get_app_version,
            load_game_save,
            save_game_save
        ])
        .run(tauri::generate_context!())
        .expect("error while running Flappy Dusk");
}

#[tauri::command]
fn set_game_fullscreen(window: tauri::Window, fullscreen: bool) -> Result<(), String> {
    window
        .set_fullscreen(fullscreen)
        .map_err(|error| error.to_string())
}

#[tauri::command]
fn get_app_version(app: tauri::AppHandle) -> String {
    app.package_info().version.to_string()
}

#[tauri::command]
fn load_game_save(directory: State<'_, SaveDirectory>) -> Result<Option<String>, String> {
    let path = directory.path.join("progress.json");
    match fs::read_to_string(path) {
        Ok(data) => Ok(Some(data)),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(format!("Could not read the game save: {error}")),
    }
}

#[tauri::command]
fn save_game_save(directory: State<'_, SaveDirectory>, data: String) -> Result<(), String> {
    if data.len() > 1_000_000 {
        return Err("The game save exceeds the maximum supported size.".into());
    }

    let _guard = directory
        .write_lock
        .lock()
        .map_err(|error| format!("Could not lock game saves: {error}"))?;
    let destination = directory.path.join("progress.json");
    let temporary = directory.path.join("progress.json.tmp");
    fs::write(&temporary, data)
        .map_err(|error| format!("Could not write the temporary game save: {error}"))?;
    fs::rename(&temporary, &destination)
        .map_err(|error| format!("Could not replace the game save: {error}"))
}
