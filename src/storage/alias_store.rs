use crate::models::{Alias, AliasSource};
use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AliasStoreError {
    #[error("Failed to read shell config file: {0}")]
    ReadError(#[source] std::io::Error),
    #[error("Failed to write shell config file: {0}")]
    WriteError(#[source] std::io::Error),
    #[error("Failed to determine shell config path")]
    ConfigPathNotFound,
    #[error("Alias not found: {0}")]
    AliasNotFound(String),
}

pub struct AliasStore {
    aliases: HashMap<String, Alias>,
    config_path: PathBuf,
    source: AliasSource,
}

impl AliasStore {
    pub fn new() -> Result<Self, AliasStoreError> {
        let (config_path, source) = Self::detect_shell_config()?;
        let mut store = Self {
            aliases: HashMap::new(),
            config_path,
            source,
        };
        store.load()?;
        Ok(store)
    }

    fn detect_shell_config() -> Result<(PathBuf, AliasSource), AliasStoreError> {
        let home = directories::BaseDirs::new()
            .ok_or(AliasStoreError::ConfigPathNotFound)?
            .home_dir()
            .to_path_buf();

        let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/bash".to_string());

        if shell.contains("zsh") {
            let zshrc = home.join(".zshrc");
            if zshrc.exists() {
                return Ok((zshrc, AliasSource::Zsh));
            }
        }

        let bashrc = home.join(".bashrc");
        if bashrc.exists() {
            return Ok((bashrc, AliasSource::Bash));
        }

        let bash_profile = home.join(".bash_profile");
        if bash_profile.exists() {
            return Ok((bash_profile, AliasSource::Bash));
        }

        Err(AliasStoreError::ConfigPathNotFound)
    }

    pub fn load(&mut self) -> Result<(), AliasStoreError> {
        let file = fs::File::open(&self.config_path).map_err(AliasStoreError::ReadError)?;
        let reader = BufReader::new(file);

        self.aliases.clear();

        for line in reader.lines() {
            let line = line.map_err(AliasStoreError::ReadError)?;
            if let Some(alias) = Alias::parse_line(&line) {
                self.aliases.insert(alias.name.clone(), alias);
            }
        }

        Ok(())
    }

    pub fn save(&self) -> Result<(), AliasStoreError> {
        let original_content =
            fs::read_to_string(&self.config_path).map_err(AliasStoreError::ReadError)?;

        let mut new_content = String::new();
        let mut alias_block_started = false;
        let mut in_alias_block = false;

        for line in original_content.lines() {
            if line.contains("# SNIPLIAS ALIASES START") {
                alias_block_started = true;
                in_alias_block = true;
                continue;
            }

            if line.contains("# SNIPLIAS ALIASES END") {
                in_alias_block = false;
                continue;
            }

            if in_alias_block {
                continue;
            }

            if !alias_block_started && line.trim().starts_with("alias ") {
                continue;
            }

            new_content.push_str(line);
            new_content.push('\n');
        }

        if !self.aliases.is_empty() {
            new_content.push_str("\n# SNIPLIAS ALIASES START\n");
            for alias in self.aliases.values() {
                new_content.push_str(&alias.to_alias_string());
                new_content.push('\n');
            }
            new_content.push_str("# SNIPLIAS ALIASES END\n");
        }

        let mut file = fs::File::create(&self.config_path).map_err(AliasStoreError::WriteError)?;
        file.write_all(new_content.as_bytes())
            .map_err(AliasStoreError::WriteError)?;

        Ok(())
    }

    pub fn list(&self) -> Vec<&Alias> {
        self.aliases.values().collect()
    }

    pub fn list_filtered(&self, query: &str) -> Vec<&Alias> {
        self.aliases
            .values()
            .filter(|a| a.matches_search(query))
            .collect()
    }

    pub fn add(&mut self, alias: Alias) -> Result<(), AliasStoreError> {
        self.aliases.insert(alias.name.clone(), alias);
        self.save()
    }

    pub fn update(&mut self, name: &str, new_alias: Alias) -> Result<(), AliasStoreError> {
        if !self.aliases.contains_key(name) {
            return Err(AliasStoreError::AliasNotFound(name.to_string()));
        }
        self.aliases.remove(name);
        self.aliases.insert(new_alias.name.clone(), new_alias);
        self.save()
    }

    pub fn delete(&mut self, name: &str) -> Result<(), AliasStoreError> {
        if self.aliases.remove(name).is_none() {
            return Err(AliasStoreError::AliasNotFound(name.to_string()));
        }
        self.save()
    }

    pub fn source(&self) -> &AliasSource {
        &self.source
    }

    pub fn source_command(&self) -> Option<String> {
        let path = self.config_path.to_string_lossy();
        Some(format!("source {}", path))
    }
}

impl Default for AliasStore {
    fn default() -> Self {
        Self::new().expect("Failed to initialize AliasStore")
    }
}
