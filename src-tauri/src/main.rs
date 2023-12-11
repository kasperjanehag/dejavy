// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
use tauri::command;
use walkdir::WalkDir;
use std::path::{ PathBuf, Path};
use std::collections::HashMap;
// use std::fs::File;
// use std::time::SystemTime;
// use std::io::Read;
// use md5;

#[derive(serde::Serialize, Debug)]
struct Image {
    relative_path: PathBuf,
    absolute_path: PathBuf,
    name: String,
    file_format: String,
    // creation_time: SystemTime,
    // size: u64,
    // resolution: Option<(u32, u32)>, // This will be None if the resolution can't be determined
    // md5_hash: String,
}

#[derive(serde::Serialize)]
struct Directory {
    name: String,
    children: Vec<FileTreeNode>,
}

#[derive(serde::Serialize)]
enum FileTreeNode {
    Directory(Directory),
    Image(Image),
}

fn paths_to_file_tree(image_map: HashMap<String, Image>) -> Vec<FileTreeNode> {
    let mut root = FileTreeNode::Directory(Directory {
        name: String::from("root"),
        children: Vec::new(),
    });



    for (path, image) in image_map {
        let mut node = &mut root;

        // Go thorugh all components of the path and update current node as we walk down the path
        // Once all components has been iterated through
        let components: Vec<_> = Path::new(&path).components().collect();

        for (i, component) in components.iter().enumerate() {
            if i == components.len() - 1 {
                continue;
            }

            let key = component.as_os_str().to_string_lossy().into_owned();
            let key_clone = key.clone();

            // Check if the current node is a directory or an image
            match node {

                // If it's a directory...
                FileTreeNode::Directory(directory) => {

                    // Look for a child directory with the same name as the current path component
                    // by iterating over the child vector, where position() returns the index of the first item where the
                    // colsure returns true.
                    let child_index = directory.children.iter().position(|child| {
                        if let FileTreeNode::Directory(dir) = child {
                            dir.name == key
                        } else {
                            false
                        }
                    });

                    // If a child directory was found...
                    match child_index {
                        Some(index) => {

                            // Set the current node to the child directory
                            node = &mut directory.children[index];
                        }

                        // If no child directory was found...
                        None => {

                            // Create a new directory with the current path component as its name
                            directory.children.push(FileTreeNode::Directory(
                                Directory {
                                    name: key_clone,
                                    children: Vec::new(),
                                }
                            ));

                            // Set the current node to the new directory
                            node = directory.children.last_mut().unwrap();
                        }
                    }
                }
                FileTreeNode::Image(_) => panic!("Image nodes should not have children"),
            };
        }
        match node {
            FileTreeNode::Directory(directory) => {
                directory.children.push(FileTreeNode::Image(image));
            }
            FileTreeNode::Image(_) => panic!("Image nodes should not have children"),
        };
    }

    match root {
        FileTreeNode::Directory(directory) => directory.children,
        FileTreeNode::Image(_) => vec![],
    }
}



#[command]
fn get_file_tree_data(path: String) -> Vec<FileTreeNode> {
    let default_extensions = vec!["jpeg", "jpg", "png"];
    let image_map = get_images(&path, &default_extensions);
    let file_tree = paths_to_file_tree(image_map);
    file_tree
}

fn get_images(search_path: &str, extensions: &[&str]) -> HashMap<String, Image> {
        
    fn create_image(absolute_path: &PathBuf, relative_path: &PathBuf) -> Option<Image> {
        // let md5_hash = compute_md5_hash(&path);
        let file_format = match absolute_path.extension() {
            Some(os_str) => match os_str.to_str() {
                Some(s) => s.to_string(),
                None => {
                    // Handle the error as needed
                    // For example, return a default value
                    "unknown".to_string()
                }
            },
            None => {
                // Handle the error as needed
                // For example, return a default value
                "unknown".to_string()
            }
        };

        let name = match absolute_path.file_name() {
            Some(os_str) => match os_str.to_str() {
                Some(s) => s.to_string(),
                None => {
                    // Handle the error as needed
                    // For example, return a default value
                    "unknown".to_string()
                }
            },
            None => {
                // Handle the error as needed
                // For example, return a default value
                "unknown".to_string()
            }
        };

        // TODO: Add these properties later
        // let metadata = std::fs::metadata(&path).ok()?;
        // let creation_time = metadata.created().ok()?;
        // let size = metadata.len();
        // let resolution = get_image_resolution(&path);

        Some(Image {
            absolute_path: absolute_path.to_path_buf(),
            relative_path: relative_path.to_path_buf(),
            name,
            // creation_time,
            // size,
            // resolution,
            // md5_hash,
            file_format,
        })
    }
    
    // fn compute_md5_hash(path: &PathBuf) -> String {
    //     let mut file = File::open(&path).unwrap();
    //     let mut buffer = Vec::new();
    //     file.read_to_end(&mut buffer).unwrap();
    //     format!("{:x}", md5::compute(&buffer))
    // }
    
    let mut map = HashMap::new();

    for entry in WalkDir::new(search_path) {
        if let Ok(entry) = entry {
            if entry.path().extension().map_or(false, |extension| extensions.contains(&extension.to_str().unwrap())) {
                let absolute_path=entry.into_path();
                let relative_path = absolute_path.strip_prefix(search_path).unwrap().to_path_buf();

                match create_image(&absolute_path, &relative_path) {
                    Some(image) => {
                        map.insert(relative_path.to_string_lossy().to_string(), image);
                    }
                    None => {}
                }
            }
        }
    }
    map
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
