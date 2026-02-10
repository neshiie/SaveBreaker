use std::io::ErrorKind;
use std::path::Path;

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
    pub fn new(filename: &str) -> Result<Self, ErrorKind> {
        if Path::new(filename).exists() {
            return Err(ErrorKind::NotFound);
        }

        let ret = Self {
            head: [0u8; 64],
            signature: FileFormat::Binary(BinarySignature::Raw),
        };

        Ok(ret)
    }
}
