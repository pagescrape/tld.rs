# tld [![Build Status](https://travis-ci.org/pagescrape/tld.rs.svg?branch=master)](https://travis-ci.org/pagescrape/tld.rs) [![Cargo](https://img.shields.io/crates/v/tld.svg)](https://crates.io/crates/tld)

Top Level domain static hash map, tld list is obtained from of iana.org


```rust,no_run
extern crate tld;

assert!(tld::exist("com"));
assert!(tld::exist("io"));
assert!(tld::exist("lt"));
assert!(tld::exist("ru"));
assert!(tld::exist("de"));

assert!(!tld::exist(""));
assert!(!tld::exist("moc"));

assert!(tld::TLD.len() > 1400);
```
