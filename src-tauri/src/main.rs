// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod utils;
use tauri::State;
use std::collections::HashMap;
use std::sync::Mutex;
use utils::frontend::{paths_to_file_tree, FileTreeNode};
use utils::image::{get_images, Image};

// Define a struct to hold the cache used by Tauri
struct TauriCache {
  image_cache: Mutex<HashMap<String, Image>>,
  md5_cache: Mutex<HashMap<String, Vec<String>>>,
}

#[tauri::command]
fn load_image_data(state: State<TauriCache>, path: String) -> Vec<FileTreeNode> {
    let default_extensions = vec!["jpeg", "jpg", "png"];
    let mut image_cache = state.image_cache.lock().unwrap();
    let image_map = get_images(&path, &default_extensions, &mut *image_cache);
    let file_tree = paths_to_file_tree(image_map, &path);
    file_tree
}

#[tauri::command]
fn get_md5_duplicates(cache: State<TauriCache>) -> HashMap<String, Vec<String>> {
    let image_cache = cache.image_cache.lock().unwrap();
    let mut md5_cache = cache.md5_cache.lock().unwrap();

    // TODO: Convert into unique set, Clear md5_cache
    md5_cache.clear();

    for (absolute_path, image) in image_cache.iter() {
        if let Some(md5_hash) = &image.md5_hash {
            md5_cache.entry(md5_hash.clone()).or_insert_with(Vec::new).push(absolute_path.clone());
        }
    }

    md5_cache.clone()
}

#[tauri::command]
fn get_image_data(absolute_path: String) -> String {
  let image_data = std::fs::read(absolute_path).expect("Failed to read image file");
  base64::encode(image_data)
}

#[tauri::command]
fn refresh_image_cache(state: State<TauriCache>, path: String) {
    // Clear the existing image cache
    state.image_cache.lock().unwrap().clear();

    // Repopulate the image cache
    load_image_data(state, path);
}

fn main() {
    tauri::Builder::default()
          .manage(TauriCache {
            image_cache: Mutex::new(HashMap::new()),
            md5_cache: Mutex::new(HashMap::new()),
        })
        .invoke_handler(tauri::generate_handler![load_image_data, get_image_data, get_md5_duplicates, refresh_image_cache])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}