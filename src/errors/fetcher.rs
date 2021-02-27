//! Messages for errors that can happen during fetching a value from the Map.
//! There are multiple ways that fetching an error from the Map can fail and
//! [TranslationFetcherError] contains the definitions for these error types.

use std::io::{Error, ErrorKind};
use std::{error, fmt};
use std::fmt::Display;

///
///
///
pub enum TranslationFetcherError {
    NoTranslationFound(String),
    NoFallbackTranslationFound(String),
    LocaleNotFound(String),
}
