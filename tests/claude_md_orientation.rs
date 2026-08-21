//! `CLAUDE.md` is orientation, and nothing may refill it.
//!
//! The 4,000 byte cap is not cosmetic: it is the gate that decides whether a
//! container working this repo gets its cwd pointed at the clone
//! (encryption4all/dobby-code#482). Above it, the repo stays a sibling directory.
//! The file this replaced was an agent-notes corpus that had already gone stale in
//! two places, so the cut is guarded here rather than left to review
//! (encryption4all/dobby-code#697).

use std::fs;
use std::path::PathBuf;

fn claude_md() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("CLAUDE.md")
}

fn read() -> String {
    let path = claude_md();
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()))
}

const MAX_BYTES: u64 = 4_000;

#[test]
fn claude_md_stays_orientation_sized() {
    let path = claude_md();
    let bytes = fs::metadata(&path)
        .unwrap_or_else(|e| panic!("cannot stat {}: {e}", path.display()))
        .len();
    assert!(
        bytes <= MAX_BYTES,
        "CLAUDE.md is {bytes} B, over the {MAX_BYTES} B cap. This file is ORIENTATION: what this \
         crate is, its position under `ibe` and `pg-core`, and the sibling repos to consider. \
         Documentation belongs at docs.postguard.eu/repos/pg-curve; a durable check belongs in the \
         rule bundle (at most 600 bytes, delivered at ~/dobby-rules.md), not here."
    );
}

/// The headings the agent-notes corpus was filed under. A byte count alone passes a
/// small junk drawer, and every one of these grew from a single line. `Overview` is
/// deliberately absent: its content is what the intro and `## Position` now carry.
const DELETED_SECTIONS: [&str; 6] = [
    "Repo quirks",
    "Security surface",
    "MSRV",
    "Build / test",
    "Clippy",
    "CI workflows",
];

#[test]
fn claude_md_has_no_heading_from_the_deleted_corpus() {
    let body = read();
    let headings: Vec<&str> = body
        .lines()
        .filter_map(|line| line.trim_start().strip_prefix('#'))
        .map(|h| h.trim_start_matches('#'))
        .map(str::trim)
        .collect();
    for section in DELETED_SECTIONS {
        // Prefix, not equality: the security section carried a parenthetical date.
        assert!(
            !headings.iter().any(|h| h.starts_with(section)),
            "CLAUDE.md has a \"{section}\" heading again, at any depth. That section went \
             with the agent-notes corpus: its content is derivable from the repo's own \
             files, a binding rule, or documentation on docs.postguard.eu now."
        );
    }
}

/// The cut corpus is not migrated and not reconstructed; it stays in git history, and
/// this file's pointer to it is the only way back. Dropping the SHA is what makes the
/// deletion unrecoverable, so it is asserted rather than trusted to review.
#[test]
fn claude_md_still_names_the_revision_holding_the_cut_corpus() {
    let body = read();
    assert!(
        body.contains("7a62ffe"),
        "CLAUDE.md no longer names 7a62ffe, the last revision carrying the agent-notes corpus \
         (`git show 7a62ffe:CLAUDE.md`). Without it a reader has no way back to what was cut."
    );
}
