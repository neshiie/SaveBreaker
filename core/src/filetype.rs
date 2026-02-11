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
    Text,
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
            signature: generate_file_format(filename, &buffer),
        };

        Ok(ret)
    }
}

const SQLITE_PREFIX: &[u8] = b"\x53\x51\x4C\x69\x74\x65\x20\x66\x6F\x72\x6D\x61\x74\x20\x33\x00";

fn generate_file_format(filename: &str, buf: &[u8; 64]) -> FileFormat {
    let parts: Vec<&str> = filename.split('.').collect();

    let opt = match parts.last() {
        None => Some(FileFormat::Binary(BinarySignature::Raw)),
        Some(v) => match *v {
            "json" => Some(FileFormat::Text(TextSignature::Json)),
            "txt" => Some(FileFormat::Text(TextSignature::Text)),
            _ => None,
        },
    };

    if let Some(v) = opt {
        return v;
    }

    match buf {
        s if s.starts_with(SQLITE_PREFIX) => FileFormat::Binary(BinarySignature::Sqlite),
        _ => FileFormat::Binary(BinarySignature::Raw),
    }
}
