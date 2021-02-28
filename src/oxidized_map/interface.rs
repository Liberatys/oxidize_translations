//! The interface for an OxidizedMap that can be implement to change the main backend of
//! Oxidized Translation.
//!

use std::io::prelude::*;

// TODO:
// Allow *_get methods to return any Yaml object rather than just a string. Could be used for 
// requests where a value is an array rather than just a string.
// Also if the developer wants to load a node key rather than a leaf.
//    - liberatys, Sat Feb 27 23:11:17 2021

/// Describes the interface for the OxidizedMap, which is the main interaction point between the
/// developer and the translation. Is abstracted from the main map to facilitate an easy shift of
/// implementation if needed or desired.
///
/// The MapInterface does not implement any methods to replace / set a value in the translation as
/// setting translations during runtime is not intended at this moment.
///
pub trait MapInterface {
    /// Convert a string representation of a configuration into an OxidizedMap
    fn from_config_str(config: String) -> Self;
    /// Load a configuration file and convert it into an OxidizedMap
    fn from_config_file(config_file_path: String) -> Self;
    /// Load a configuration from a Struct that implements  [Read]
    fn from_config_stream(config_stream: &mut dyn Read) -> Self;
    /// Returns an Option for the given Key, if no value is found for the given key it will return
    /// None. Otherwise it will return Some(value). If no Locale is provided the default_locale
    /// will be taken from the configuration for the Map.
    fn optional_get(&self, key: &str, locale: Option<&str>) -> Option<String>;
    /// Enforces a return value for a given key. It essentially wraps [MapInterface::optional_get] and
    /// returns a message to inform the user/developer that the given key was not found in the data
    /// provided by the Map if  [MapInterface::optional_get] returns None.
    fn enforced_get(&self, key: &str, locale: Option<&str>) -> String;
    /// Load the translations from disk and compile them into the needed data structure.
    fn load(&mut self) -> std::io::Result<()>;
}
