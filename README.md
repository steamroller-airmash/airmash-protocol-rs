
# Airmash Rust Protocol Library

![ghactions] [![Latest Version]][crates.io] ![docs]

[Latest Version]: https://img.shields.io/crates/v/airmash-protocol.svg
[crates.io]: https://crates.io/crates/airmash-protocol
[docs]: https://docs.rs/airmash-protocol/badge.svg
[ghactions]: https://img.shields.io/github/checks-status/steamroller-airmash/airmash-protocol-rs/master

This library allows for communicating with the 
[airmash](https://airma.sh) server or for communicating
with clients using the same protocol. It is meant to be used
with a protocol backend such as the
[protocol-v5](https://crates.io/crate/airmash-protocol-v5) crate.

At the moment, this library only supports rust nightly due
to the use of a few nightly-only features.

## Getting the library

To use this library, just add 
```
airmash-protocol = "0.4"
```
to your `Cargo.toml`. This will give you the
latest version of the library.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
