# pg-curve

A fork of [`zkcrypto/bls12_381`](https://github.com/zkcrypto/bls12_381) that adds
target-group (Gt) serialization. Upstream declined the serialization standard
([zkcrypto/bls12_381#12](https://github.com/zkcrypto/bls12_381/pull/12)), so the fork
carries it. `no_std`, `#![deny(unsafe_code)]`, published to crates.io by hand.

## Position

The lowest layer of PostGuard's Rust stack. `ibe` does its BLS12-381 arithmetic here,
`pg-core` sits on `ibe`, and `postguard-js`, `postguard-dotnet` and
`postguard-business` sit on `pg-core`. So a change to curve arithmetic or to a
serialized encoding reaches the whole PostGuard family, and nothing in this repo's
tests will say so. The coupling is a published crates.io version range rather than a
build: a change here reaches `pg-core` only through an `ibe` release that `pg-core` is
then bumped to. Encoding changes are governed by `postguard`'s `COMPATIBILITY.md`, and
a break in one surfaces in `postguard-e2e` before it surfaces here.

Being a fork is the other half of the position. Ours is the Gt serialization
(`pairings.rs`, `fp12.rs`) and `hash_to_curve/map_scalar.rs`; the rest is upstream
`bls12_381`. A bug outside those files is an upstream bug, and a change to them is
divergence someone carries at the next upstream merge.

## Sibling repos to consider

- `encryption4all/ibe` — the only consumer of this crate. (`ibs` is not one: it is on
  `curve25519-dalek`.)
- `encryption4all/postguard` — `pg-core`, the PKG and `COMPATIBILITY.md`; the root of
  the family.
- `encryption4all/postguard-docs` — `docs.postguard.eu/repos/pg-curve`, where this
  repo's documentation lives.

One company, two GitHub orgs. `encryption4all` is the vehicle the PostGuard research
project used to apply for grants, kept as an org after Yivi bought PostGuard;
`privacybydesign` is the Yivi/IRMA lineage. The split is historical, not
organisational: same company, same maintainers, same review conventions. We are
maintainers here rather than upstream contributors.

## Where the operational knowledge is

Not in this file. Documentation belongs at `docs.postguard.eu/repos/pg-curve`; a
durable check belongs in the rule bundle, which the host lands in the next container at
`~/dobby-rules.md`. A test in `tests/claude_md_orientation.rs` holds this file to
4,000 bytes.

The agent-notes corpus this file used to be (MSRV lessons, the clippy allow list, the
CI job layout, a 2026-07-07 security review) is in git history at `7a62ffe`, the last
revision carrying it (`git show 7a62ffe:CLAUDE.md`). It had already rotted there: it
said the repo carries no committed `Cargo.lock`, which stopped being true at `d699009`,
and it named `ibe`'s feature set wrongly.
