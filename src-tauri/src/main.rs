// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
use tauri::command;
use walkdir::WalkDir;
use std::path::PathBuf;
use std::collections::HashMap;


#[derive(serde::Serialize, Debug)]
struct Item {
    id: u32,
    name: String,
    children: Option<Vec<Item>>,
}

#[derive(Default)]
struct Node {
    id: Option<u32>,
    children: HashMap<String, Node>,
}

fn paths_to_items(paths: Vec<PathBuf>) -> Vec<Item> {
    let mut root = Node::default();
    let mut id_counter = 1;

    for path in paths {
        let mut node = &mut root;
        for component in path.components() {
            let key = component.as_os_str().to_string_lossy().into_owned();
            node = node.children.entry(key).or_insert_with(Node::default);
        }
        node.id = Some(id_counter);
        id_counter += 1;
    }

    fn convert(node: Node, name: String) -> Item {
        Item {
            id: node.id.unwrap(),
            name,
            children: if node.children.is_empty() {
                None
            } else {
                Some(node.children.into_iter().map(|(name, node)| convert(node, name)).collect())
            },
        }
    }

    root.children.into_iter().map(|(name, node)| convert(node, name)).collect()
}

#[command]
fn get_data() -> Vec<Item> {
    let default_path = "/Users/kasper_janehag/Library/CloudStorage/OneDrive-McKinsey&Company/Documents/repos/dejavy/tests";
    let paths = list_paths(default_path, true);
    let file_tree = paths_to_items(paths);
    file_tree
}

#[command]
fn list_paths_in_test_dir() -> Vec<String> {
    let default_path = "/Users/kasper_janehag/Library/CloudStorage/OneDrive-McKinsey&Company/Documents/repos/dejavy/tests";
    let paths = list_paths(default_path, true);
    paths.into_iter().map(|path| path.to_string_lossy().into_owned()).collect()
}

fn list_paths(base_path: &str, return_relative: bool) -> Vec<PathBuf> {
    WalkDir::new(base_path)
        .into_iter()
        .filter_map(Result::ok)
        .map(|entry| {
            if return_relative {
                entry.path().strip_prefix(base_path).unwrap().to_path_buf()
            } else {
                entry.into_path()
            }
        })
        .collect()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_data,list_paths_in_test_dir])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


// use std::path::Path;
// use walkdir::WalkDir;

