//!
//!
//!

use super::file_reader::FileReader;
use std::collections::HashMap;
use std::fs;
use std::io;
use yaml_rust::{Yaml, YamlLoader};

/// A wrapper for loading all files within a folder. Takes a folder path and enables the loading
/// of the files in the folder into a HashMap containing the folder_path as the key and the parsed
/// yaml as the value. See: [LocaleFolder::load]
pub struct LocaleFolder {
    locale_folder_path: String,
}

impl LocaleFolder {
    /// Create a new LocaleFolder with a path to the folder on disk.
    pub fn new(locale_folder_path: String) -> Self {
        Self { locale_folder_path }
    }

    /// Iterates over the files in a folder then returns them as a map containing the
    /// file_path as the key and the parsed yaml as a value.
    /// Will return an Err on invalid file path or issues when parsing
    /// the file content of a locale file
    pub fn load(&self) -> io::Result<HashMap<String, Yaml>> {
        let mut locale_folder_map: HashMap<String, Yaml> = HashMap::new();
        let file_entries = fs::read_dir(&self.locale_folder_path)?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()?;

        for file_path in file_entries {
            let str_path = file_path.to_str().unwrap().to_string();
            let file_content = FileReader::new(str_path.clone()).read();
            match file_content {
                Ok(content) => {
                    let yaml_content = YamlLoader::load_from_str(&content).unwrap();
                    // Ignore the return Option, as we don't care about it! There will never in a
                    // normal system be a file with exact same file path!
                    let _ = locale_folder_map.insert(str_path, yaml_content[0].clone());
                }
                Err(error) => {
                    return Err(error);
                }
            }
        }

        Ok(locale_folder_map)
    }
}
