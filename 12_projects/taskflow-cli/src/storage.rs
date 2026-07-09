use std::fs;
use std::path::PathBuf;

use crate::models::TaskStore;

const DATA_FILE: &str = "tasks.json";

pub fn data_path() -> PathBuf {
    PathBuf::from(DATA_FILE)
}

pub fn load() -> Result<TaskStore, String> {
    let path = data_path();
    if !path.exists() {
        return Ok(TaskStore::default());
    }

    let content = fs::read_to_string(&path)
        .map_err(|e| format!("File padhne me error: {}", e))?;

    serde_json::from_str(&content).map_err(|e| format!("JSON parse error: {}", e))
}

pub fn save(store: &TaskStore) -> Result<(), String> {
    let json = serde_json::to_string_pretty(store)
        .map_err(|e| format!("JSON banane me error: {}", e))?;

    fs::write(data_path(), json).map_err(|e| format!("File likhne me error: {}", e))
}
