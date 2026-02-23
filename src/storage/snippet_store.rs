#![allow(dead_code)]
use crate::models::Snippet;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum SnippetStoreError {
    #[error("Failed to read snippets file: {0}")]
    ReadError(#[source] std::io::Error),
    #[error("Failed to write snippets file: {0}")]
    WriteError(#[source] std::io::Error),
    #[error("Failed to parse snippets: {0}")]
    ParseError(#[source] serde_json::Error),
    #[error("Snippet not found: {0}")]
    SnippetNotFound(String),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct SnippetStoreData {
    snippets: Vec<Snippet>,
}

pub struct SnippetStore {
    snippets: HashMap<Uuid, Snippet>,
    data_path: PathBuf,
}

impl SnippetStore {
    pub fn new() -> Result<Self, SnippetStoreError> {
        let data_path = Self::get_data_path()?;

        let store = Self {
            snippets: HashMap::new(),
            data_path,
        };

        if store.data_path.exists() {
            let mut store = store;
            store.load()?;
            Ok(store)
        } else {
            if let Some(parent) = store.data_path.parent() {
                fs::create_dir_all(parent).map_err(SnippetStoreError::WriteError)?;
            }
            store.save()?;
            Ok(store)
        }
    }

    fn get_data_path() -> Result<PathBuf, SnippetStoreError> {
        let base_dirs =
            directories::ProjectDirs::from("com", "sniplias", "sniplias").ok_or_else(|| {
                SnippetStoreError::ReadError(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Could not find data directory",
                ))
            })?;
        Ok(base_dirs.data_dir().join("snippets.json"))
    }

    pub fn load(&mut self) -> Result<(), SnippetStoreError> {
        let content = fs::read_to_string(&self.data_path).map_err(SnippetStoreError::ReadError)?;

        let data: SnippetStoreData = if content.trim().is_empty() {
            SnippetStoreData { snippets: vec![] }
        } else {
            serde_json::from_str(&content).map_err(SnippetStoreError::ParseError)?
        };

        self.snippets = data.snippets.into_iter().map(|s| (s.id, s)).collect();

        Ok(())
    }

    pub fn save(&self) -> Result<(), SnippetStoreError> {
        let data = SnippetStoreData {
            snippets: self.snippets.values().cloned().collect(),
        };

        let content =
            serde_json::to_string_pretty(&data).map_err(|e| SnippetStoreError::ParseError(e))?;

        if let Some(parent) = self.data_path.parent() {
            fs::create_dir_all(parent).map_err(SnippetStoreError::WriteError)?;
        }

        fs::write(&self.data_path, content).map_err(SnippetStoreError::WriteError)?;

        Ok(())
    }

    pub fn list(&self) -> Vec<&Snippet> {
        self.snippets.values().collect()
    }

    pub fn list_filtered(&self, query: &str) -> Vec<&Snippet> {
        self.snippets
            .values()
            .filter(|s| s.matches_search(query))
            .collect()
    }

    pub fn get(&self, id: &Uuid) -> Option<&Snippet> {
        self.snippets.get(id)
    }

    pub fn add(&mut self, snippet: Snippet) -> Result<(), SnippetStoreError> {
        self.snippets.insert(snippet.id, snippet);
        self.save()
    }

    pub fn update(&mut self, id: Uuid, snippet: Snippet) -> Result<(), SnippetStoreError> {
        if !self.snippets.contains_key(&id) {
            return Err(SnippetStoreError::SnippetNotFound(id.to_string()));
        }
        self.snippets.insert(id, snippet);
        self.save()
    }

    pub fn delete(&mut self, id: &Uuid) -> Result<(), SnippetStoreError> {
        if self.snippets.remove(id).is_none() {
            return Err(SnippetStoreError::SnippetNotFound(id.to_string()));
        }
        self.save()
    }

    pub fn data_path(&self) -> &PathBuf {
        &self.data_path
    }
}

impl Default for SnippetStore {
    fn default() -> Self {
        Self::new().expect("Failed to initialize SnippetStore")
    }
}
