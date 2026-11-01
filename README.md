# facturx

Factur-X for the French e-invoicing reform, in Rust.

Version 0.1 detects which Factur-X profile an invoice declares in its guideline
identifier. Known profiles are recognised, including identifiers in their
ZUGFeRD spelling.

```rust
let xml = std::fs::read_to_string("factur-x.xml")
    .expect("failed to read factur-x.xml");

match facturx::Profile::from_xml(&xml) {
    Some(profile) => println!("profile: {}", profile.name()),
    None => println!("not a readable CII invoice"),
}
```

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
