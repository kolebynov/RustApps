use std::{collections::VecDeque, path::{PathBuf}, fs};

use log::{debug, info, warn};

use crate::common::{Document, DocumentContent};

pub fn traverse(roots: &[PathBuf]) -> impl Iterator<Item = Document> {
    FileSystemTraverser { 
        items: roots.iter()
            .map(|x| FileSystemItem { path: x.to_owned() })
            .collect()
    }
}

struct FileSystemTraverser {
    items: VecDeque<FileSystemItem>,
}

struct FileSystemItem {
    path: PathBuf,
}

impl Iterator for FileSystemTraverser {
    type Item = Document;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(current_item) = self.items.pop_front() {
            debug!("Traversing file system item {}", current_item.path.display());

            match current_item.path.is_file() {
                true => {
                    return Some(Document {
                        uri: current_item.path.to_str().unwrap().to_string(),
                        content: DocumentContent::File(current_item.path),
                    });
                },
                false => {
                    match fs::read_dir(&current_item.path) {
                        std::io::Result::Ok(dir_iter) => {
                            for item in dir_iter {
                                let item = item.unwrap();
                                self.items.push_back(FileSystemItem { path: item.path() });
                            }
                        },
                        std::io::Result::Err(err) =>
                            warn!("Failed to enumerate directory {}. Error: {}", current_item.path.display(), err)
                    }
                }
            }
        }

        None
    }
}