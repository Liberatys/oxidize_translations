use std::collections::HashMap;
use yaml_rust::Yaml;

// TODO:
//  Strip language definition from hash
//    - liberatys, Sun Feb 28 16:43:04 2021
///
pub fn filter_translation_files(
    translation_map: HashMap<String, Yaml>,
    locales: &Vec<String>,
) -> HashMap<String, Vec<Yaml>> {
    let mut locale_map: HashMap<String, Vec<Yaml>> = HashMap::new();
    for (k, v) in translation_map {
        match &v {
            Yaml::Hash(value) => {
                let locale_definition =
                    value.keys().next().unwrap().clone().into_string().unwrap();
                if locales.contains(&locale_definition) {
                    match locale_map.get_mut(&locale_definition) {
                        None => {
                            let _ = locale_map.insert(
                                locale_definition.clone(),
                                vec![value[&Yaml::String(locale_definition)].clone()],
                            );
                        }
                        Some(yaml_vector) => {
                            yaml_vector.push(value[&Yaml::String(locale_definition)].clone());
                        }
                    }
                }
            }
            _ => {}
        }
    }

    locale_map
}
