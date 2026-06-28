//! Smoke tests mirroring the canonical suite.
//!
//! The canonical suite asserts only two things: the export is a list and it is
//! non-empty. The type assertion that the export is an immutable list of
//! strings is satisfied here structurally by the slice type.

use binary_extensions::BINARY_EXTENSIONS as EXT;

/// The export is a slice of `&str` and is non-empty.
#[test]
fn main_is_nonempty_list() {
    let _: &[&str] = EXT;
    assert!(!EXT.is_empty(), "list must be non-empty");
}

/// Every element is a string slice.
#[test]
fn elements_are_str() {
    for e in EXT {
        let _: &str = e;
    }
}

/// The accessor returns the same slice as the constant.
#[test]
fn accessor_matches_constant() {
    assert_eq!(binary_extensions::binary_extensions(), EXT);
}
