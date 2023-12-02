// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
use tauri::command;

#[derive(serde::Serialize)]
struct Item {
    id: u32,
    icon: String,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_open: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    children: Option<Vec<Item>>,
}

#[command]
fn get_data() -> Vec<Item> {
    vec![
        Item {
            id: 1,
            icon: "mdi-plus-outline".to_string(),
            name: "File 1".to_string(),
            is_open: None,
            children: None,
        },
        Item {
            id: 2,
            icon: "mdi-plus-outline".to_string(),
            name: "Folder 1".to_string(),
            is_open: Some(false),
            children: Some(vec![
                Item {
                    id: 3,
                    icon: "mdi-plus-outline".to_string(),
                    name: "File 2".to_string(),
                    is_open: None,
                    children: None,
                },
            ]),
        },
    ]
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
