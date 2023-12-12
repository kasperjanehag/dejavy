use walkdir::WalkDir;
use std::collections::HashMap;
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;

#[derive(serde::Serialize, Debug)]
pub struct Image {
    relative_path: PathBuf,
    absolute_path: PathBuf,
    name: String,
    file_format: String,
    // creation_time: SystemTime,
    // size: u64,
    // resolution: Option<(u32, u32)>, // This will be None if the resolution can't be determined
    md5_hash: Option<String>,
}

pub fn get_images(search_path: &str, extensions: &[&str]) -> HashMap<String, Image> {
        
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

        let md5_hash = match compute_md5_hash(&absolute_path) {
            Ok(hash) => Some(hash),
            Err(e) => {
                eprintln!("Failed to compute MD5 hash for {:?}: {}", absolute_path, e);
                None
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
            md5_hash,
            file_format,
        })
    }
    
    fn compute_md5_hash(path: &PathBuf) -> std::io::Result<String> {
        let mut file = File::open(&path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        Ok(format!("{:x}", md5::compute(&buffer)))
    }
    
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
