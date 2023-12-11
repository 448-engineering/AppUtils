#![forbid(unsafe_code)]
#![forbid(missing_docs)]

//! Collection of macro utilities commonly used
//!
//!
//! By default all features are enabled but you can disable then using `default-features = false` in
//! Cargo.toml file.
//! The features are:
//! 1. `app_dir` - enables getting the user's home directory path, concatenating a new path
//! and returning [std::path::PathBuf] or error
//! 2. `utf8` - enables getting the user's home directory path, concatenating a new path
//! and returning a Utf8 path `camino::Utf8PathBuf` or error
//!
///
mod errors;
pub use errors::*;

mod dirs;
pub use dirs::*;

mod other;
pub use other::*;
