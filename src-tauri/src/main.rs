// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
use tauri::command;

#[derive(serde::Serialize)]
struct Item {
    id: u32,
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

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
