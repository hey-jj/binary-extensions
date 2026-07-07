//! Golden parity check against the checked-in extension list.
//!
//! The fixture `binary-extensions.json` holds the canonical 262-entry list.
//! This test asserts the crate reproduces it exactly: same entries, same order,
//! same casing. It subsumes length, contents, order, and casing checks.

use std::path::PathBuf;

/// Read the fixture file as a single string.
fn read_fixture() -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/binary-extensions.json");
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {}", path.display(), e))
}

/// Parse the fixture into ordered string tokens borrowed from `raw`.
///
/// The fixture is a flat JSON array of plain ASCII strings with no escapes.
/// This keys on the array body: it isolates the text between the outer `[` and
/// `]`, splits on commas, then strips whitespace and the surrounding quotes from
/// each element. A quoted token outside the brackets is ignored, and a token
/// without both quotes fails the test instead of silently shifting indices.
fn parse_fixture(raw: &str) -> Vec<&str> {
    let open = raw.find('[').expect("fixture missing opening bracket");
    let close = raw.rfind(']').expect("fixture missing closing bracket");
    let body = &raw[open + 1..close];
    body.split(',')
        .map(|chunk| {
            let token = chunk.trim();
            token
                .strip_prefix('"')
                .and_then(|t| t.strip_suffix('"'))
                .unwrap_or_else(|| panic!("fixture element is not a quoted string: {:?}", token))
        })
        .collect()
}

#[test]
fn golden_matches_fixture_exactly() {
    let raw = read_fixture();
    let fixture = parse_fixture(&raw);
    let ours: Vec<&str> = binary_extensions::BINARY_EXTENSIONS.to_vec();

    assert_eq!(fixture.len(), 262, "fixture drifted from the curated list");
    assert_eq!(ours.len(), fixture.len(), "length mismatch vs fixture");

    for (i, (a, b)) in ours.iter().zip(fixture.iter()).enumerate() {
        assert_eq!(
            a, b,
            "mismatch at index {}: ours={:?} fixture={:?}",
            i, a, b
        );
    }
}
