use std::path::{PathBuf};

pub enum DocumentContent {
    None,
    String(String),
    File(PathBuf),
}

pub struct Document {
    pub uri: String,
    pub content: DocumentContent,
}