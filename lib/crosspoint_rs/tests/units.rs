//! Byte formatting, which is pure and so needs no host.

use crosspoint_rs::units::Units;

/// Separators every three digits, counting from the right. A six-figure heap
/// number is the whole reason this exists.
#[test]
fn digits_are_grouped_in_threes() {
    let bytes = Units::Bytes;

    assert_eq!(bytes.format(0), "0 B");
    assert_eq!(bytes.format(999), "999 B");
    assert_eq!(bytes.format(1_000), "1,000 B");
    assert_eq!(bytes.format(12_345), "12,345 B");
    assert_eq!(bytes.format(199_204), "199,204 B");
    assert_eq!(bytes.format(1_048_576), "1,048,576 B");
}

/// Kilobytes round to nearest, so a figure just under 1 KB does not read as
/// "0 KB" and imply the heap is exhausted.
#[test]
fn kilobytes_round_to_nearest() {
    let kb = Units::Kilobytes;

    assert_eq!(kb.format(0), "0 KB");
    assert_eq!(kb.format(511), "0 KB");
    assert_eq!(
        kb.format(512),
        "1 KB",
        "half a kilobyte rounds up, not down"
    );
    assert_eq!(kb.format(1_024), "1 KB");
    assert_eq!(kb.format(1_536), "2 KB");
    assert_eq!(kb.format(199_204), "195 KB");
}

/// The unit is always written, so the two scales can never be read as one
/// another — 195 and 199,204 describe the same heap.
#[test]
fn both_scales_name_themselves() {
    assert!(Units::Bytes.format(199_204).ends_with(" B"));
    assert!(Units::Kilobytes.format(199_204).ends_with(" KB"));
}

/// Cycling returns to where it started, so tapping a row twice is a no-op.
#[test]
fn cycling_twice_returns_to_the_start() {
    let start = Units::default();

    assert_ne!(start, start.next());
    assert_eq!(start, start.next().next());
}

/// A negative figure keeps its sign in front of the grouped digits. No heap
/// reading is negative, but the formatter is now shared and should not produce
/// something like "1,-024".
#[test]
fn a_negative_value_keeps_its_sign() {
    assert_eq!(Units::Bytes.format(-12_345), "-12,345 B");
}
