use std::collections::HashMap;
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;
use walkdir::WalkDir;
use rayon::prelude::*;

#[derive(serde::Serialize, Debug, Clone)]
pub struct Image {
    relative_path: PathBuf,
    absolute_path: PathBuf,
    name: String,
    file_format: String,
    // creation_time: SystemTime,
    // size: u64,
    // resolution: Option<(u32, u32)>, // This will be None if the resolution can't be determined
    pub md5_hash: Option<String>,
}

pub fn get_images(search_path: &str, extensions: &[&str], cache: &mut HashMap<String, Image>) -> HashMap<String, Image> {
        
    fn create_image(absolute_path: &PathBuf, relative_path: &PathBuf) -> Option<Image> {

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
    
    // Create a vector of directory entries for the given search path, filtering for files with the specified extensions
    let entries: Vec<_> = WalkDir::new(search_path)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().extension().map_or(false, |extension| extensions.contains(&extension.to_str().unwrap())))
        .collect();

    // Use rayon's parallel iterator to create a HashMap of new images that are not already in the cache
    let new_images: HashMap<_, _> = entries.par_iter().filter_map(|entry| {

        // Get the absolute and relative paths of the entry
        let absolute_path = entry.path().to_path_buf();
        let relative_path = absolute_path.strip_prefix(search_path).unwrap().to_path_buf();
    
        // If the image is not already in the cache, create a new image and add it to the HashMap
        if !cache.contains_key(&absolute_path.to_string_lossy().to_string()) {
            create_image(&absolute_path, &relative_path).map(|image| (absolute_path.to_string_lossy().to_string(), image))
        } else {
            None
        }
    }).collect();
    
    // Extend the cache with the new images
    cache.extend(new_images);
    
    // Return a clone of the updated cache
    cache.clone()
}
