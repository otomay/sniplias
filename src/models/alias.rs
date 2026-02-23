#![allow(dead_code)]
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Alias {
    pub id: Uuid,
    pub name: String,
    pub command: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub source_file: AliasSource,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AliasSource {
    Bash,
    Zsh,
}

impl AliasSource {
    pub fn as_str(&self) -> &'static str {
        match self {
            AliasSource::Bash => "bash",
            AliasSource::Zsh => "zsh",
        }
    }

    pub fn from_shell_name(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "bash" => Some(AliasSource::Bash),
            "zsh" => Some(AliasSource::Zsh),
            _ => None,
        }
    }
}

impl Alias {
    pub fn new(name: String, command: String, source: AliasSource) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            command,
            description: None,
            created_at: now,
            updated_at: now,
            source_file: source,
        }
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn to_alias_string(&self) -> String {
        format!("alias {}='{}'", self.name, self.command)
    }

    pub fn parse_line(line: &str) -> Option<Self> {
        let trimmed = line.trim();
        if !trimmed.starts_with("alias ") {
            return None;
        }

        let rest = trimmed.strip_prefix("alias ")?;
        let eq_pos = rest.find('=')?;
        let name = rest[..eq_pos].trim().to_string();

        let command_part = &rest[eq_pos + 1..];
        let command = if (command_part.starts_with('\'') && command_part.ends_with('\''))
            || (command_part.starts_with('"') && command_part.ends_with('"'))
        {
            command_part[1..command_part.len() - 1].to_string()
        } else {
            command_part.to_string()
        };

        Some(Self {
            id: Uuid::new_v4(),
            name,
            command,
            description: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            source_file: AliasSource::Bash,
        })
    }

    pub fn matches_search(&self, query: &str) -> bool {
        let query_lower = query.to_lowercase();
        self.name.to_lowercase().contains(&query_lower)
            || self.command.to_lowercase().contains(&query_lower)
            || self
                .description
                .as_ref()
                .map(|d| d.to_lowercase().contains(&query_lower))
                .unwrap_or(false)
    }
}
