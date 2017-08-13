#![feature(plugin)]
#![plugin(phf_macros)]
#![warn(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    variant_size_differences
)]

//!  TLD - top level domain static map, list is obtained from iana.org.

extern crate phf;

mod tld;

pub use tld::TLD;

/// Convenience function to check if given TLD exists in
/// IANA official TLD list.
///
/// https://data.iana.org/TLD/tlds-alpha-by-domain.txt
pub fn exist(s: &str) -> bool {
    TLD.contains_key(s)
}

#[test]
fn test_tld() {
    use tld::TLD;
    assert!(TLD.get("aaa").is_some());
    assert!(TLD.get("#33dawaaa").is_none());
    assert!(TLD.get("aco").is_some());
    assert!(TLD.get("uk").is_some());
    assert!(TLD.get("ye").is_some());
    assert!(TLD.get("com").is_some());
    assert!(TLD.get("de").is_some());
    assert!(TLD.get("fr").is_some());
    assert!(TLD.get("ag").is_some());
    assert!(TLD.get("ru").is_some());
    assert!(TLD.get("nl").is_some());
    assert!(TLD.get("lt").is_some());
    assert!(TLD.get("amex").is_some());
    assert!(TLD.get("zw").is_some());
}

#[test]
fn test_exist() {
    assert!(exist("fr"));
    assert!(exist("de"));
    assert!(exist("zw"));
    assert!(!exist("a9292zw"));
}
