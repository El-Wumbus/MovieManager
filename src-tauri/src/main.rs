mod Metadata;
mod key;
mod libary;
mod Display;

use directories::UserDirs;
use key::omdb_key;
use libary::scan;
use Display::display;

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
    display::make_cards()
}
