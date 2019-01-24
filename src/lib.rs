#![feature(plugin)]
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

include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

/// Convenience function to check if given TLD exists in
/// IANA official TLD list.
///
/// https://data.iana.org/TLD/tlds-alpha-by-domain.txt
pub fn exist(s: &str) -> bool {
    TLD.contains(s)
}

#[test]
fn test_tld() {
    use TLD;
    assert!(TLD.get_key("aaa").is_some());
    assert!(TLD.get_key("#33dawaaa").is_none());
    assert!(TLD.get_key("aco").is_some());
    assert!(TLD.get_key("uk").is_some());
    assert!(TLD.get_key("ye").is_some());
    assert!(TLD.get_key("com").is_some());
    assert!(TLD.get_key("de").is_some());
    assert!(TLD.get_key("fr").is_some());
    assert!(TLD.get_key("ag").is_some());
    assert!(TLD.get_key("ru").is_some());
    assert!(TLD.get_key("nl").is_some());
    assert!(TLD.get_key("lt").is_some());
    assert!(TLD.get_key("amex").is_some());
    assert!(TLD.get_key("zw").is_some());
}

#[test]
fn test_exist() {
    assert!(exist("fr"));
    assert!(exist("de"));
    assert!(exist("zw"));
    assert!(!exist("a9292zw"));
    assert!(!exist("mcd"));
}
