use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct SaveInput {
    pub path: PathBuf,
    pub text: String,
}

impl SaveInput {
    #[allow(dead_code)]
    pub fn len_chars(&self) -> usize {
        self.text.len()
    }

    pub fn preview(&self, n: usize) -> &str {
        let end = self.text.len().min(n);
        &self.text[..end]
    }
}
