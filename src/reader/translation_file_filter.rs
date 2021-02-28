use std::collections::HashMap;
use yaml_rust::Yaml;

// TODO:
//  Filter out all files that don't match the available locales
//    - liberatys, Sun Feb 28 16:43:04 2021
///
pub fn filter_translation_files(mut translation_map: HashMap<String, Yaml>,
    locales: Vec<String>) -> HashMap<String, Yaml> {
    
    translation_map
}
