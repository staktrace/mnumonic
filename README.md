mnumonic
===
[![Build Status](https://travis-ci.org/staktrace/mnumonic.svg?branch=master)](https://travis-ci.org/staktrace/mnumonic)
[![Crate](https://img.shields.io/crates/v/mnumonic.svg)](https://crates.io/crates/mnumonic)

This is a tiny Rust library that allows you to convert binary byte data to and from a phrase that is memorable for a human.
It is useful for cases where you need a human to transfer certain information (e.g. an opaque identifier) between systems where copy/paste may not be readily available.
Instead of asking them to type in an opaque number like 3735928559, you can instead have them type in "sweet pump second tree".

Currently only English is supported, but the code is written to allow other languages too. PRs to add word files for other languages are welcome.

See API documentation at [docs.rs](https://docs.rs/staktrace/mnumonic/) for full details.

Example usage
---

```rust
use mnumonic;
let opaque_identifier : u32 = 0xDEADBEEF;
let human_readable = mnumonic::encode_u32_joined(opaque_identifier);
println!("Instead of remembering '{}' (difficult), you can remember '{}' (easier)", opaque_identifier, human_readable);
let back_to_identifier = mnumonic::decode_u32_joined(&human_readable).unwrap();
assert_eq!(opaque_identifier, back_to_identifier);
```
