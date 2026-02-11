use anyhow::{anyhow, Context, Result};
use base64::engine::general_purpose::STANDARD;
use base64::Engine;

pub fn decode_to_bytes(s: &str) -> Result<Vec<u8>> {
    let cleaned: String = s.chars().filter(|c| !c.is_whitespace()).collect();

    STANDARD
        .decode(cleaned.as_bytes())
        .with_context(|| "invalid base64 input")
}

pub fn decode_to_utf8(s: &str) -> Result<String> {
    let bytes = decode_to_bytes(s)?;
    String::from_utf8(bytes).map_err(|e| anyhow!("decoded bytes are not valid UTF-8: {e}"))
}

