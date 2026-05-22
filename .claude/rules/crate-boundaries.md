---
description: Enforces strict crate dependency boundaries
globs: crates/**/*.rs, crates/**/Cargo.toml
---

# Crate Dependency Boundaries

`ibus-vie-im` must NEVER depend on IBus, DBus, zbus, tokio, or any I/O.

Dependency graph (no cycles allowed):

```
ibus-vie-cli    → ibus-vie-im
ibus-vie-engine → ibus-vie-im → vi (crates.io)
```

- `ibus-vie-im` depends only on the `vi` crate (pure Vietnamese text transformation). No async, no networking, no filesystem.
- `ibus-vie-engine` is the ONLY crate that may depend on `zbus`, `tokio`, `tracing`, `tracing-subscriber`, `serde`, `toml`, `clap`, `thiserror`.
- `ibus-vie-cli` depends only on `ibus-vie-im` and `clap`.

When modifying `Cargo.toml` of any crate, verify these boundaries are preserved.
