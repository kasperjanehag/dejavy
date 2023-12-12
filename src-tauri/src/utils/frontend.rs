use std::collections::HashMap;
use std::path::Path;
use super::image::Image;



#[derive(serde::Serialize)]
pub struct Directory {
    name: String,
    children: Vec<FileTreeNode>,
}

#[derive(serde::Serialize)]
pub enum FileTreeNode {
    Directory(Directory),
    Image(Image),
}

pub fn paths_to_file_tree(image_map: HashMap<String, Image>) -> Vec<FileTreeNode> {
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

