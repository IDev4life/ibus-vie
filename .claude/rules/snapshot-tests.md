---
description: Snapshot test format and conventions
globs: tests/snapshot/**/*.txt, crates/ibus-vie-im/tests/**/*.rs
---

# Snapshot Test Format

Files in `tests/snapshot/` are tab-separated: `input<TAB>expected_output`

```
vieetj	việt
tieengs	tiếng
```

Rules:

- One test case per line
- Lines starting with `#` are comments
- Separator is a literal TAB character, not spaces
- Input is the raw keystroke sequence (ASCII)
- Expected output is the final composed Vietnamese string
- Each line tests a single word/syllable (no spaces in input unless testing word boundaries)

## Test structure

Tests are **integration tests** in `crates/ibus-vie-im/tests/`:

- `snapshot.rs` — reads `tests/snapshot/*.txt` and runs each line through the engine
- `telex.rs` — Telex-specific named tests
- `vni.rs` — VNI-specific named tests

Source files (`telex.rs`, `vni.rs`) contain NO inline `#[cfg(test)]` modules — all tests live in the integration test directory.

## When adding new cases

1. Add a line to the corresponding `tests/snapshot/<method>.txt`
2. Optionally add a named `#[test]` in `crates/ibus-vie-im/tests/<method>.rs` for complex scenarios
3. Run `cargo test --workspace` to verify

## Running specific tests

```bash
cargo test -p ibus-vie-im --test snapshot   # snapshot .txt runner only
cargo test -p ibus-vie-im --test telex      # telex named tests only
cargo test -p ibus-vie-im --test vni        # vni named tests only
cargo test -p ibus-vie-im                   # all ibus-vie-im tests
```
