// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod utils;
use tauri::command;
use utils::frontend::{paths_to_file_tree, FileTreeNode};
use utils::image::get_images;

#[command]
fn get_file_tree_data(path: String) -> Vec<FileTreeNode> {
    let default_extensions = vec!["jpeg", "jpg", "png"];
    let image_map = get_images(&path, &default_extensions);
    let file_tree = paths_to_file_tree(image_map);
    file_tree
}


#[tauri::command]
fn get_image_data(absolute_path: String) -> String {
  let image_data = std::fs::read(absolute_path).expect("Failed to read image file");
  base64::encode(image_data)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_file_tree_data,get_image_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
