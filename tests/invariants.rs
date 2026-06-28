//! Invariant checks that pin the data and guard against drift.
//!
//! These cover properties the canonical smoke suite leaves untested:
//! uniqueness, casing, order, membership, and format.

use binary_extensions::BINARY_EXTENSIONS as EXT;
use std::collections::HashSet;

/// No duplicate entries.
#[test]
fn all_unique() {
    let set: HashSet<&&str> = EXT.iter().collect();
    assert_eq!(set.len(), EXT.len(), "duplicate extension(s) present");
}

/// The list has exactly 262 entries.
#[test]
fn length_is_262() {
    assert_eq!(EXT.len(), 262);
}

/// No empty entries, leading dots, or whitespace.
#[test]
fn no_empty_no_leading_dot_no_whitespace() {
    for e in EXT {
        assert!(!e.is_empty(), "empty extension entry");
        assert!(
            !e.starts_with('.'),
            "extension must not have leading dot: {e:?}"
        );
        assert!(
            !e.contains(char::is_whitespace),
            "no whitespace allowed: {e:?}"
        );
    }
}

/// Every entry is ASCII.
#[test]
fn all_ascii() {
    for e in EXT {
        assert!(e.is_ascii(), "non-ASCII entry: {e:?}");
    }
}

/// `DS_Store` is the only entry with uppercase letters.
#[test]
fn casing_preserved_ds_store_is_the_only_uppercase() {
    let upper: Vec<&&str> = EXT
        .iter()
        .filter(|e| e.chars().any(|c| c.is_ascii_uppercase()))
        .collect();
    assert_eq!(
        upper,
        vec![&"DS_Store"],
        "casing drift: exactly one uppercase entry (DS_Store) expected, got {upper:?}"
    );
}

/// Representative extensions are present.
#[test]
fn known_members_present() {
    for ext in [
        "3dm", "png", "jpg", "jpeg", "pdf", "zip", "exe", "DS_Store", "zipx",
    ] {
        assert!(EXT.contains(&ext), "expected member missing: {ext}");
    }
}

/// Common text extensions are absent.
#[test]
fn obvious_text_extensions_absent() {
    for ext in ["txt", "md", "json", "js", "ts", "html", "css", "csv"] {
        assert!(
            !EXT.contains(&ext),
            "text extension wrongly included: {ext}"
        );
    }
}

/// First entry is `3dm`, last is `zipx`.
#[test]
fn endpoints_match() {
    assert_eq!(EXT.first(), Some(&"3dm"));
    assert_eq!(EXT.last(), Some(&"zipx"));
}

/// The curated order is preserved and not re-sorted.
#[test]
fn order_is_not_resorted() {
    let mut sorted = EXT.to_vec();
    sorted.sort_unstable();
    assert_ne!(
        EXT.to_vec(),
        sorted,
        "list appears re-sorted; curated order must be preserved"
    );
}
