#![feature(plugin)]
#![plugin(phf_macros)]

#[macro_use]
extern crate phf;

mod tld;

pub use tld::TLD;

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
