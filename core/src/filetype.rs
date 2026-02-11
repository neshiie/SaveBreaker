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

    if opt.is_some() {
        return opt.unwrap();
    }

    match buf {
        [
            0x53,
            0x51,
            0x4C,
            0x69,
            0x74,
            0x65,
            0x20,
            0x66,
            0x6F,
            0x72,
            0x6D,
            0x61,
            0x74,
            0x20,
            0x33,
            0x00,
            ..,
        ] => FileFormat::Binary(BinarySignature::Sqlite),
        _ => FileFormat::Binary(BinarySignature::Raw),
    }
}
