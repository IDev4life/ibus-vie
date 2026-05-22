---
description: Code quality gates before committing
globs: "**/*.rs"
---

# Code Quality

Before committing any Rust change, all three must pass:

```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test --workspace
```

Or use the shortcut: `make check`

- Zero warnings policy — clippy warnings are denied.
- Do not use `#[allow(...)]` unless there is a clear justification (e.g., intentionally unused field for future use).
- Prefer `_` prefix for intentionally unused parameters over `#[allow(unused)]`.
- `ibus-vie-im` and `ibus-vie-vi` use `#![forbid(unsafe_code)]` — never add unsafe to these crates.
