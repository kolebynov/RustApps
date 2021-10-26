use std::path::{PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub task: TaskSettings,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TaskSettings {
    pub traverse: TraverseSettings,
    pub indexing_api: IndexingApiSettings,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IndexingApiSettings {
    pub uri: String,
    pub token: String,
    pub kb_id: i32,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TraverseSettings {
    pub roots: Vec<PathBuf>,
}