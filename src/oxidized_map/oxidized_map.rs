//! The main interaction point for the developer to interact with the loaded translations
//!

use super::interface::MapInterface;
use crate::config::config;
use crate::reader::locale_folder;
use crate::reader::translation_file_filter;
use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use yaml_rust::Yaml;

/// A Map implementation that contains the loaded translations for the application and implements
/// the methods needed to load different parts of the translation.
///
#[derive(Debug, Default, PartialEq)]
pub struct Map {
    ///
    configuration: config::Configuration,
    locale_map: HashMap<String, Vec<Yaml>>,
}

impl Map {
    /// Create a new instance of the OxidizedMap by a given configuration
    pub fn new(configuration: config::Configuration) -> Self {
        let mut map = Map::default();
        map.configuration = configuration;
        map
    }
}

// TODO:
//  Handle unwrap
//    - liberatys, Sun Feb 28 03:30:37 2021

impl MapInterface for Map {
    fn from_config_str(config: String) -> Self {
        let configuration = config::Configuration::from_str(config).unwrap();
        let mut map = Map::default();
        map.configuration = configuration;
        map
    }

    fn from_config_file(file_path: String) -> Self {
        let configuration = config::Configuration::from_file(file_path)
            .unwrap()
            .unwrap();
        let mut map = Map::default();
        map.configuration = configuration;
        map
    }

    fn from_config_stream(config_stream: &mut dyn Read) -> Self {
        let configuration = config::Configuration::from_read(config_stream)
            .unwrap()
            .unwrap();
        let mut map = Map::default();
        map.configuration = configuration;
        map
    }

    fn optional_get(&self, key: &str, locale: Option<&str>) -> Option<String> {
        None
    }

    fn enforced_get(&self, key: &str, locale: Option<&str>) -> String {
        String::from("")
    }

    fn load(&mut self) -> io::Result<()> {
        let translation_map =
            locale_folder::LocaleFolder::new(self.configuration.locale_folder.clone()).load()?;
        let locale_map = translation_file_filter::filter_translation_files(
            translation_map,
            &self.configuration.available_locales,
        );
        println!("{:?}", locale_map);
        Ok(())
    }
}
