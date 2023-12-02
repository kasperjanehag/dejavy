// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
use tauri::command;
use walkdir::WalkDir;

#[derive(serde::Serialize)]
struct Item {
    id: u64,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    children: Option<Vec<Item>>,
}

#[command]
fn get_data() -> Vec<Item> {
    vec![
        Item {
            id: 1,
            name: "File 1".to_string(),
            children: None,
        },
        Item {
            id: 2,
            name: "Folder 1".to_string(),
            children: Some(vec![
                Item {
                    id: 3,
                    name: "File 2".to_string(),
                    children: None,
                },
            ]),
        },
    ]
}

#[command]
fn list_paths() -> Vec<String> {
    let path = "/Users/kasper_janehag/Library/CloudStorage/OneDrive-McKinsey&Company/Documents/repos/dejavy/tests";
    WalkDir::new(path)
        .into_iter()
        .filter_map(Result::ok)
        .map(|entry| entry.path().to_string_lossy().into_owned())
        .collect()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_data,list_paths])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


// use std::path::Path;
// use walkdir::WalkDir;

