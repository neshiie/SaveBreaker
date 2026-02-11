use anyhow::{Context, Result};
use core::types::SaveInput;
use std::{
    fs,
    path::{Path, PathBuf},
};

// new implementation
// reading into memory
pub fn load_text<P: AsRef<Path>>(path: P) -> Result<SaveInput> {
    let filepath = path.as_ref();
    let raw = fs::read_to_string(filepath)
        .with_context(|| format!("failed to read file as UTF-8: {}", filepath.display()))?;

    SaveInput::new(PathBuf::from(filepath), raw.trim().to_string())
}
