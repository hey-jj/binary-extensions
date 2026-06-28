//! Curated list of binary file extensions.
//!
//! This crate ships a single immutable list of file extensions that denote
//! binary files (archives, images, audio, video, office documents,
//! executables, fonts, RAW camera formats, and similar). It carries no logic.
//! Read [`BINARY_EXTENSIONS`] or call [`all`].
//!
//! Use it to classify a path as binary by its extension. Strip any leading dot
//! from the path extension first, then check membership. The list stores bare
//! tokens such as `"png"`, never `".png"`.
//!
//! # Data shape
//!
//! - 262 entries, all unique.
//! - Hand-curated order. The list is roughly alphabetical but is not the output
//!   of any standard sort, so it is stored verbatim.
//! - Every entry is lowercase except `"DS_Store"`, the macOS Finder metadata
//!   file. The casing of that entry is preserved exactly.
//! - All entries are ASCII with no leading dot, whitespace, or empty strings.
//!
//! # Example
//!
//! ```
//! use binary_extensions::BINARY_EXTENSIONS;
//!
//! assert!(BINARY_EXTENSIONS.contains(&"png"));
//! assert!(!BINARY_EXTENSIONS.contains(&"txt"));
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![no_std]

mod data;

pub use data::BINARY_EXTENSIONS;

/// Return the list of binary file extensions.
///
/// This is an ergonomic accessor for [`BINARY_EXTENSIONS`]. It returns the same
/// static slice.
///
/// # Example
///
/// ```
/// assert_eq!(binary_extensions::all(), binary_extensions::BINARY_EXTENSIONS);
/// assert!(binary_extensions::all().contains(&"zip"));
/// ```
#[must_use]
pub fn all() -> &'static [&'static str] {
    BINARY_EXTENSIONS
}
