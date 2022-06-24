
# Rust Airmash Protocol Library

![ghactions] [![Latest Version]][crates.io] ![docs]

[Latest Version]: https://img.shields.io/crates/v/airmash-protocol.svg
[crates.io]: https://crates.io/crates/airmash-protocol
[docs]: https://docs.rs/airmash-protocol/badge.svg
[ghactions]: https://img.shields.io/github/checks-status/steamroller-airmash/airmash-protocol-rs/master

This library allows for communicating with the [airmash](https://airmash.online) servers or 
for communicating with clients using the same protocol. By default it provides serialization
and deserialization for the airmash v5 protocol under the `v5` module allows for serializing
and deserializing all provided types using `serde` if the `"serde"` feature is enabled.

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
