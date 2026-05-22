---
description: Rules for FSM engine changes and testing requirements
globs: crates/ibus-vie-im/src/**/*.rs
---

# FSM Engine Rules

## Every change requires a test

Any modification to `telex.rs`, `vni.rs`, `viqr.rs`, `buffer.rs`, or `engine.rs` MUST include at least one new test case — either a unit test in the file or a line in the corresponding `tests/snapshot/*.txt`.

## trait Engine contract

All engines implement `trait Engine` with these methods:

- `fn key(&mut self, ev: KeyEvent) -> Action` — core handler
- `fn reset(&mut self)` — clear preedit
- `fn preedit(&self) -> &str` — current composed text
- `fn feed_str(&mut self, input: &str) -> String` — default impl: feed chars, return committed + preedit (used in tests)

Return values:

- `Action::Update` — preedit changed, redisplay
- `Action::Commit(String)` — send text to app, the committed string includes the trigger character
- `Action::PassThrough` — engine doesn't handle this key

## Buffer is shared

All engines use `Buffer` from `buffer.rs`. Do not duplicate buffer logic in engine files.

## Verify with CLI after changes

After modifying any engine, verify manually:

```bash
cargo run -p ibus-vie-cli -- --method telex --input "vieetj"
cargo run -p ibus-vie-cli -- --method telex --trace --input "vieetj"  # trace FSM steps
```
