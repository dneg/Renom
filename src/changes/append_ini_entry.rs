use std::{fmt::Display, path::PathBuf};

#[derive(Debug, PartialEq, Clone)]
pub struct AppendIniEntry {
    pub path: PathBuf,
    pub section: String,
    pub key: String,
    pub value: String,
}

impl AppendIniEntry {
    pub fn new(
        path: impl Into<PathBuf>,
        section: impl Into<String>,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        Self {
            path: path.into(),
            section: section.into(),
            key: key.into(),
            value: value.into(),
        }
    }
}

impl Display for AppendIniEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Append key [{}] with value [{}] to section [{}] in INI file [{}]",
            &self.key,
            &self.value,
            &self.section,
            &self.path.to_str().unwrap_or("invalid Unicode path")
        )
    }
}