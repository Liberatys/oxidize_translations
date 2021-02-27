//!
//!
//!
//!
use std::fs;

///
pub struct FileReader {
    file_path: String,
}

impl FileReader {
    pub fn new(file_path: String) -> Self {
        Self { file_path }
    }

    pub fn read(&self) -> std::io::Result<String> {
        let file_content = String::from_utf8_lossy(&fs::read(&self.file_path)?).to_string();
        Ok(file_content)
    }
}
