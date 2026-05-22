---
description: Enforces strict crate dependency boundaries
globs: crates/**/*.rs, crates/**/Cargo.toml
---

# Crate Dependency Boundaries

`ibus-vie-im` and `ibus-vie-vi` must NEVER depend on IBus, DBus, zbus, tokio, or any I/O.

Dependency graph (no cycles allowed):

```
ibus-vie-cli    → ibus-vie-im
ibus-vie-engine → ibus-vie-im → ibus-vie-vi
```

- `ibus-vie-vi` is a leaf crate — zero workspace dependencies, zero external deps.
- `ibus-vie-im` depends only on `ibus-vie-vi`. No async, no networking, no filesystem.
- `ibus-vie-engine` is the ONLY crate that may depend on `zbus`, `tokio`, `tracing`, `serde`, `toml`.
- `ibus-vie-cli` depends only on `ibus-vie-im` and `clap`.

When modifying `Cargo.toml` of any crate, verify these boundaries are preserved.
