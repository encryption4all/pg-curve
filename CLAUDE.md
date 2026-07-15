# Agent notes (migrated from the dobby memory repo)

## Overview
`encryption4all/pg-curve` is a fork of `zkcrypto/bls12_381` with target-group (Gt)
serialization added. Used by `ibe` (with features `groups`, `pairings`, `alloc`,
`bits`). `edition = "2021"`, `#![no_std]`, `#![deny(unsafe_code)]`. Dual licensed
MIT/Apache-2.0.

## Repo quirks
- Fork remote `upstream` points to `Wassasin/irmaseal-curve`; always pass
  `--repo encryption4all/pg-curve` to `gh` commands, or they target upstream
  instead.
- Issues are disabled on this repo; `gh issue create` errors ("the repository has
  disabled issues"). Surface findings as a draft PR instead, or open them on
  `encryption4all/dobby`.
- No `pr-title.yml`.
- Manual release process, no automation.

## Security surface (deep-reviewed 2026-07-07, clean)
The only fork-specific code worth auditing is Gt/target-group serialization
(`pairings.rs` `to/from_(un)compressed`, `fp12.rs`) and
`hash_to_curve/map_scalar.rs`; the rest is upstream `bls12_381` 0.8.0. Verdict:
sound. Both Gt deserialization paths gate on `Fp12::is_element()` (checks `x^r ==
1`, order-r subgroup membership), so no invalid or small-subgroup element
deserializes. `map_scalar` is standard RFC 9380 (L=48 wide reduction);
`expand_msg` DST-oversize handling is RFC-compliant. Two low-severity issues are
tracked privately, don't re-file.

## MSRV
- Rust 1.80.0 (raised from 1.56.0).
- `rayon-core` 1.13.0 (transitive via the `criterion` dev-dep) requires Rust
  1.80+.
- `ci.yml` and `rust-toolchain.toml` both pin `1.80.0`.
- All 127 tests pass on 1.80.0.
- **Lock-file lesson:** there's no committed `Cargo.lock` (library convention),
  so CI always resolves latest deps; dev-dep chains (criterion -> rayon ->
  crossbeam) are what actually drive MSRV breakage. Always `rm Cargo.lock`
  before testing locally to mirror CI. Concretely, a fresh resolution pulls
  `zeroize 1.9.0`, which requires `edition2024` and fails to build on 1.80.0; the
  "127 tests pass" claim above holds only after `cargo update -p zeroize
  --precise 1.8.1` (pin `<1.9`).

## Build / test
- `cargo check` (library only) works on very old Rust.
- `cargo test --features experimental,zeroize` runs 127 tests.
- `cargo build --benches --examples --all-features` for bitrot checking.
- No-std targets: `thumbv6m-none-eabi`, `wasm32-unknown-unknown`, `wasm32-wasi`.

## Clippy
Many pre-existing warnings come from upstream `bls12_381` code. `lib.rs` has
`#![allow(...)]` for: too_many_arguments, many_single_char_names,
suspicious_arithmetic_impl, needless_borrow, op_ref, clone_on_copy,
wrong_self_convention, bool_assert_comparison, identity_op,
needless_borrows_for_generic_args, deprecated (generic_array), unexpected_cfgs.
The `deprecated` warnings come from `digest 0.9` using `generic_array 0.14`;
fixing requires bumping to `digest 0.10+` (an API-changing bump).

## CI workflows
- `ci.yml`: lint (fmt), test (ubuntu/macOS/windows), no-std, bitrot, doc-links,
  rustfmt.
- `lints-stable.yml`: Clippy MSRV (PRs only).
- `lints-beta.yml`: Clippy beta (push only, continue-on-error).
- Bitrot, doc-links, fmt, and clippy-MSRV jobs read `rust-toolchain.toml` (no
  explicit toolchain); updating that file fixes them.
- Lint, test, and no-std jobs pin `toolchain: 1.80.0` explicitly with `override:
  true`, so `rust-toolchain.toml` does not govern them; bumping the MSRV means
  editing both.
