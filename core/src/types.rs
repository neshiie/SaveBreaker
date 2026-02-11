use anyhow::Result;
use std::path::PathBuf;

use crate::filetype::FileMetadata;

#[derive(Debug, Clone)]
pub struct SaveInput {
    pub path: PathBuf,
    pub text: String,
    pub metadata: FileMetadata,
}

impl SaveInput {
    pub fn new(filepath: PathBuf, text: String) -> Result<Self> {
        let path = filepath.to_str().unwrap_or("");
        let metadata = FileMetadata::new(path)?;

        Ok(Self {
            path: filepath,
            text,
            metadata,
        })
    }

    pub fn len_chars(&self) -> usize {
        self.text.len()
    }

    pub fn preview(&self, n: usize) -> &str {
        let end = self.text.len().min(n);
        &self.text[..end]
    }
}
