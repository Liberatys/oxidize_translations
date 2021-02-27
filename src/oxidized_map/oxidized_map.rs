//! The main interaction point for the developer to interact with the loaded translations
//!

use super::interface::MapInterface;
use std::io::prelude::*;

/// A Map implementation that contains the loaded translations for the application and implements
/// the methods needed to load different parts of the translation.
///
#[derive(Debug, Default)]
pub struct Map {

}

impl MapInterface for Map {

    fn from_config_str(config: String) -> Self {
        Self {}
    }

    fn from_config_file(file_path: String) -> Self {
        Self {}
    }

    fn from_config_stream(config_stream: &dyn Read) -> Self {
        Self {}
    }

    fn optional_get(&self, key: &str, locale: Option<&str>) -> Option<String> {
        None
    }

    fn enforced_get(&self, key: &str, locale: Option<&str>) -> String {
        String::from("")
    }
}
