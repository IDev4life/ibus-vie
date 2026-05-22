# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
# Build
cargo build --release          # release binary
make build                     # same via Makefile

# Test
cargo test --workspace         # all tests
cargo test -p ibus-vie-im      # single crate
cargo test telex_snapshot      # single test by name

# Code quality
cargo fmt --all                # format
cargo clippy --all-targets --all-features -- -D warnings  # lint
make check                     # fmt + lint + test

# Dev install (no root needed)
make install-user              # installs to ~/.local, then: ibus restart

# Debug FSM without IBus
cargo run -p ibus-vie-cli -- --method telex
cargo run -p ibus-vie-cli -- --method vni --input "vieetj"
cargo run -p ibus-vie-cli -- --method telex --trace --input "vieetj"  # trace each FSM step
```

## Architecture

Four crates with strict dependency boundaries:

```
ibus-vie-cli  ──►  ibus-vie-im  ──►  ibus-vie-vi
ibus-vie-engine ──►  ibus-vie-im
                 ──►  zbus, tokio, tracing, serde, toml
```

**`ibus-vie-vi`** (leaf, no deps) — static data: Vietnamese alphabet, syllable structure, tone placement tables. No I/O.

**`ibus-vie-im`** — pure FSM, no IBus, no I/O. Three engines (`TelexEngine`, `VniEngine`, `ViqrEngine`) all implement `trait Engine`:

- `fn key(&mut self, ev: KeyEvent) -> Action` — core keystroke handler
- `fn feed_str(&mut self, input: &str) -> String` — convenience for testing (feed full string, return committed + preedit)
- Returns `Action::Update`, `Action::Commit(String)`, or `Action::PassThrough`
- `Buffer` in `buffer.rs` holds raw keystrokes + composed Vietnamese; all engines share it

**`ibus-vie-engine`** — the actual binary spawned by `ibus-daemon`. Implements `org.freedesktop.IBus.Engine` DBus interface via `zbus`. `engine_impl.rs` translates IBus keyval/state integers → `KeyEvent` → delegates to `dyn Engine`. Loads config from `~/.config/ibus-vie/config.toml`.

**`ibus-vie-cli`** — dev/debug binary. Two modes:

- One-shot: `--input "vieetj"` → prints result and exits
- Interactive: reads stdin line by line, prints composed output
- `--trace` flag prints each FSM step

## Configuration

File: `~/.config/ibus-vie/config.toml` (optional, defaults apply if missing)

```toml
method = "telex"       # telex | vni | viqr
tone_style = "new"     # new (hòa) | old (hoà)
```

## Key design rule

`ibus-vie-im` and `ibus-vie-vi` must never depend on IBus, DBus, or any I/O. All IBus glue lives exclusively in `ibus-vie-engine`. This separation allows the entire input logic to be tested without a running daemon.

## Snapshot tests

`tests/snapshot/telex.txt` (and `vni.txt`, `viqr.txt`) are tab-separated input/expected files. Adding a test case requires only adding a line — no Rust needed:

```
vieetj	việt
```

The Rust test runner in `ibus-vie-im` reads these files and calls `engine.feed_str(input)` against each line. Uses `insta` crate for snapshot assertion.

## Adding a new input method

1. Create `crates/ibus-vie-im/src/<name>.rs` implementing `trait Engine`
2. Re-export from `src/lib.rs`
3. Wire into `IbusEngineImpl::new()` match in `crates/ibus-vie-engine/src/ibus/engine_impl.rs`
4. Add engine XML entry to `data/vie.xml.in`
5. Add `tests/snapshot/<name>.txt`

## IBus integration flow

`ibus-daemon` reads `vie.xml` → spawns `ibus-engine-vie` binary → calls `process_key_event(keyval, keycode, state)` on DBus → `engine_impl.rs` translates to `KeyEvent` → FSM returns `Action` → engine sends `UpdatePreedit`/`CommitText` signals back over DBus.

## Debugging

```bash
# FSM only (no daemon)
RUST_LOG=debug cargo run -p ibus-vie-cli -- --method telex

# With IBus running
RUST_LOG=ibus_vie=debug ibus restart
journalctl --user -f | grep ibus-vie

# Inspect DBus
busctl --user introspect org.freedesktop.IBus /org/freedesktop/IBus
dbus-monitor --session "interface='org.freedesktop.IBus.Engine'"
```
