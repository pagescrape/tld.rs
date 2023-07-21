# tld [![Rust](https://github.com/pagescrape/tld.rs/actions/workflows/rust.yml/badge.svg)](https://github.com/pagescrape/tld.rs/actions/workflows/rust.yml)

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
