//! Golden parity check against the checked-in extension list.
//!
//! The fixture `binary-extensions.json` holds the canonical 262-entry list.
//! This test asserts the crate reproduces it exactly: same entries, same order,
//! same casing. It subsumes length, contents, order, and casing checks.

use std::path::PathBuf;

/// Load the fixture as an ordered list of strings.
///
/// The fixture is a flat JSON array of plain strings. No entry contains an
/// escaped quote, so splitting on the double-quote character and taking every
/// other chunk yields each string token in order.
fn load_fixture() -> Vec<String> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/binary-extensions.json");
    let raw =
        std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    raw.split('"')
        .skip(1)
        .step_by(2)
        .map(str::to_owned)
        .collect()
}

#[test]
fn golden_matches_fixture_exactly() {
    let fixture = load_fixture();
    let ours: Vec<String> = binary_extensions::BINARY_EXTENSIONS
        .iter()
        .map(|s| s.to_string())
        .collect();

    assert_eq!(fixture.len(), 262, "fixture drifted from the curated list");
    assert_eq!(ours.len(), fixture.len(), "length mismatch vs fixture");

    for (i, (a, b)) in ours.iter().zip(fixture.iter()).enumerate() {
        assert_eq!(a, b, "mismatch at index {i}: ours={a:?} fixture={b:?}");
    }
    assert_eq!(ours, fixture, "full ordered list differs from fixture");
}
