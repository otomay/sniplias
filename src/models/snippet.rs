use chrono::{DateTime, Utc};
use regex::Regex;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Snippet {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub command: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SnippetVariable {
    pub name: String,
    pub default_value: Option<String>,
}

impl Snippet {
    pub fn new(title: String, command: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title,
            description: None,
            command,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn extract_variables(&self) -> Vec<SnippetVariable> {
        let re = Regex::new(r"\{\{(\w+)(?::([^}]*))?\}\}").unwrap();
        let mut variables = Vec::new();
        let mut seen = std::collections::HashSet::new();

        for cap in re.captures_iter(&self.command) {
            let name = cap[1].to_string();
            if seen.insert(name.clone()) {
                let default_value = cap.get(2).map(|m| m.as_str().to_string());
                variables.push(SnippetVariable {
                    name,
                    default_value,
                });
            }
        }

        variables
    }

    pub fn render_command(&self, values: &std::collections::HashMap<String, String>) -> String {
        let re = Regex::new(r"\{\{(\w+)(?::[^}]*)?\}\}").unwrap();
        let mut result = self.command.clone();

        for cap in re.captures_iter(&self.command) {
            let var_name = &cap[1];
            let full_match = cap.get(0).unwrap().as_str();
            if let Some(value) = values.get(var_name) {
                result = result.replace(full_match, value);
            }
        }

        result
    }

    pub fn matches_search(&self, query: &str) -> bool {
        let query_lower = query.to_lowercase();
        self.title.to_lowercase().contains(&query_lower)
            || self
                .description
                .as_ref()
                .map(|d| d.to_lowercase().contains(&query_lower))
                .unwrap_or(false)
            || self.command.to_lowercase().contains(&query_lower)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_variables() {
        let snippet = Snippet::new(
            "Test".to_string(),
            "git clone {{repo}} {{branch:main}}".to_string(),
        );
        let vars = snippet.extract_variables();
        assert_eq!(vars.len(), 2);
        assert_eq!(vars[0].name, "repo");
        assert_eq!(vars[0].default_value, None);
        assert_eq!(vars[1].name, "branch");
        assert_eq!(vars[1].default_value, Some("main".to_string()));
    }

    #[test]
    fn test_render_command() {
        let snippet = Snippet::new(
            "Test".to_string(),
            "git clone {{repo}} -b {{branch}}".to_string(),
        );
        let mut values = std::collections::HashMap::new();
        values.insert(
            "repo".to_string(),
            "https://github.com/user/repo".to_string(),
        );
        values.insert("branch".to_string(), "develop".to_string());

        let rendered = snippet.render_command(&values);
        assert_eq!(
            rendered,
            "git clone https://github.com/user/repo -b develop"
        );
    }
}
