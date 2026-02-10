use std::{fs, path::Path};

use anyhow::{Context, Result};

pub fn read_text_trimmed<P: AsRef<Path>>(path: P) -> Result<String> {
    /// store the file path reference
    let filepath = path.as_ref();

    let raw = fs::read_to_string(filepath)
        .with_context(|| format!("failed to read file as UTF-8: {}", filepath.display()))?;

    Ok(raw.trim().to_string())
}
