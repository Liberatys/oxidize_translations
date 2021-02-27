//!
//!
//!
//!
//!

use std::io::prelude::*;

///
#[derive(Default)]
pub struct Configuration {
    default_locale: String,
    available_locales: Vec<String>,
    locale_folder: String,
}

impl Configuration {

    pub fn from_str(config_str: String) {

    }

    pub fn from_file(file_path: String) {

    }

    pub fn from_read(read_stream: &dyn Read) {

    }
}
