//! The main interaction point for the developer to interact with the loaded translations
//!

use super::interface::MapInterface;
use crate::config::config;
use std::io::prelude::*;
use crate::reader::locale_folder;

/// A Map implementation that contains the loaded translations for the application and implements
/// the methods needed to load different parts of the translation.
///
#[derive(Debug, Default, PartialEq)]
pub struct Map {
    ///
    configuration: config::Configuration,
}

// TODO:
//  Handle unwrap
//    - liberatys, Sun Feb 28 03:30:37 2021

impl MapInterface for Map {

    fn from_config_str(config: String) -> Self {
        let configuration = config::Configuration::from_str(config).unwrap();
        Self {
            configuration,
        }
    }

    fn from_config_file(file_path: String) -> Self {
        let configuration = config::Configuration::from_file(file_path).unwrap().unwrap();
        Self {
            configuration,
        }
    }

    fn from_config_stream(config_stream: &mut dyn Read) -> Self {
        let configuration = config::Configuration::from_read(config_stream).unwrap().unwrap();
        Self {
            configuration,
        }
    }

    fn optional_get(&self, key: &str, locale: Option<&str>) -> Option<String> {
        None
    }

    fn enforced_get(&self, key: &str, locale: Option<&str>) -> String {
        String::from("")
    }

    fn load(&mut self) {
        let _translation_map = locale_folder::LocaleFolder::new(self.configuration.locale_folder.clone());
    }
}
