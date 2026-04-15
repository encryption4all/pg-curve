# <p align="center"><img src="./img/pg_logo.svg" height="128px" alt="PostGuard" /></p>

> For full documentation, visit [docs.postguard.eu](https://docs.postguard.eu/repos/pg-curve).

A fork of the [BLS12-381 pairing-friendly elliptic curve construction](https://github.com/zkcrypto/bls12_381) crate, adding target group serialization. This fork exists because the upstream maintainers do not wish to merge the proposed serialization standard ([zkcrypto/bls12_381#12](https://github.com/zkcrypto/bls12_381/pull/12)). The crate tracks upstream with the `gt-serialisation` branch merged.

In the PostGuard ecosystem, pg-curve is the lowest-level building block. The `ibe` crate depends on it for all BLS12-381 curve arithmetic.

## Development

Build the crate with all features:

```sh
cargo build --all-features
```

Run tests:

```sh
cargo test
```

## Releasing

New versions are published manually to [crates.io](https://crates.io/crates/pg_curve). Bump the version in `Cargo.toml`, commit, tag, and run `cargo publish`. See `RELEASES.md` for version history.

## License

MIT OR Apache-2.0
