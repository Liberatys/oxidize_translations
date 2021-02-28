//!
//!
//!
//!
//!

use std::io::prelude::*;
use yaml_rust::{Yaml, YamlLoader};
use crate::reader::file_reader;

///
#[derive(Default, Debug, PartialEq)]
pub struct Configuration {
    default_locale: String,
    available_locales: Vec<String>,
    locale_folder: String,
}

// TODO:
//  Refactor!
//    - liberatys, Sun Feb 28 02:56:56 2021
//
pub fn extract_config_from_content(config_content: &Yaml) -> Option<Configuration> {
    let mut configuration = Configuration::default();
    match config_content["default_locale"].as_str() {
        Some(default_locale) => {
            configuration = configuration.default_locale(default_locale.to_string());
        },
        None => {
            return None;
        }
    }

    match config_content["locale_folder"].as_str() {
        Some(locale_folder) => {
            configuration = configuration.locale_folder(locale_folder.to_string());
        },
        None => {
            return None;
        }
    }

    match config_content["available_locales"].as_vec() {
        Some(available_locales_vec) => {
            let mut available_locales: Vec<String> = Vec::new();
            for entry in available_locales_vec { 
                available_locales.push(entry.as_str().unwrap().to_string());
            }
            configuration = configuration.available_locales(available_locales);
        },
        None => {
            return None;
        }
    }

    Some(configuration)
}

impl Configuration {
    pub fn from_str(config_str: String) -> Option<Self> {
        let config_content = &YamlLoader::load_from_str(&config_str).unwrap()[0];
        extract_config_from_content(config_content)
    }

    pub fn from_file(file_path: String) -> std::io::Result<Option<Self>> {
        let config_content = &YamlLoader::load_from_str(&file_reader::FileReader::new(file_path).read()?).unwrap()[0];
        Ok(extract_config_from_content(config_content))
    }

    pub fn from_read(read_stream: &mut dyn Read) -> std::io::Result<Option<Self>> {
        let mut read_content = String::new();
        let _ = read_stream.read_to_string(&mut read_content)?;
        let config_content = &YamlLoader::load_from_str(&read_content).unwrap()[0];
        Ok(extract_config_from_content(config_content))
    }

    pub fn default_locale(mut self, locale: String) -> Self {
        self.default_locale = locale;
        return self;
    }

    pub fn locale_folder(mut self, folder: String) -> Self {
        self.locale_folder = folder;
        return self;
    }

    pub fn available_locales(mut self, available_locales: Vec<String>) -> Self {
        self.available_locales = available_locales;
        return self;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_config_from_str_with_valid_data() {
        let config_str = "
                default_locale: 
                    en
                locale_folder:
                    '/folder/path/'
                available_locales:
                    - en
                    - de
                    - fr
                ";

        let config = Configuration::from_str(config_str.to_string()).unwrap();
        assert_eq!(config.default_locale, "en");
        assert_eq!(config.available_locales, vec!["en", "de", "fr"]);
        assert_eq!(config.locale_folder, "/folder/path/");
    }

    #[test]
    fn test_load_config_from_str_with_invalid_data() {
        let config_str = "
                locale_folder:
                    '/folder/path/'
                available_locales:
                    - fr
                ";

        let config = Configuration::from_str(config_str.to_string());
        assert_eq!(config, None);
    }
}
