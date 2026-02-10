use anyhow::Result;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Clone)]
pub enum FileFormat {
    Binary(BinarySignature),
    Text(TextSignature),
}

#[derive(Clone, Debug)]
pub enum BinarySignature {
    Raw,
    Sqlite,
}

#[derive(Debug, Clone)]
pub enum TextSignature {
    Json,
}

#[derive(Debug, Clone)]
pub struct FileMetadata {
    pub head: [u8; 64],
    pub signature: FileFormat,
}

impl FileMetadata {
    pub fn new(filename: &str) -> Result<FileMetadata> {
        let mut f = File::open(filename)?;
        let mut buffer = [0u8; 64];
        let file_head = f.read(&mut buffer)?;

        if file_head == 0 {
            return Err(anyhow::anyhow!("File empty!"));
        }

        let ret = Self {
            head: buffer,
            signature: FileFormat::Binary(BinarySignature::Raw),
        };

        Ok(ret)
    }
}
