mod Metadata;
mod key;
use Metadata::metadata;
use key::omdb_key;


#[macro_use]
extern crate savefile_derive;

#[cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_movies_cards])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_movies_cards() -> String
{
    String::from("Test")
}