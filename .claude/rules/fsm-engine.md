---
description: Rules for FSM engine changes and testing requirements
globs: crates/ibus-vie-im/src/**/*.rs
---

# FSM Engine Rules

## Every change requires a test

Any modification to `telex.rs`, `vni.rs`, `buffer.rs`, or `engine.rs` MUST include at least one new test case — either a unit test in the file or a line in the corresponding `tests/snapshot/*.txt`.

## trait Engine contract

All engines implement `trait Engine` using the template method pattern:

Required methods (must implement):

- `fn buffer(&self) -> &Buffer` — access internal buffer
- `fn buffer_mut(&mut self) -> &mut Buffer` — mutable access to internal buffer

Optional override:

- `fn process_char(&mut self, c: char) -> Option<Action>` — engine-specific key interception (e.g., VNI digit keys). Return `None` to fall through to default handling.

Default implementations (provided by trait):

- `fn key(&mut self, ev: KeyEvent) -> Action` — core handler (backspace replay, escape, commit triggers)
- `fn reset(&mut self)` — clear preedit
- `fn preedit(&self) -> &str` — current composed text
- `fn feed_str(&mut self, input: &str) -> String` — feed chars, return committed + preedit (used in tests)

Return values:

- `Action::Update` — preedit changed, redisplay
- `Action::Commit(String)` — send text to app, the committed string includes the trigger character
- `Action::PassThrough` — engine doesn't handle this key

## Buffer wraps vi-rs

All engines use `Buffer` from `buffer.rs`, which wraps `vi::methods::IncrementalBuffer`. Vietnamese text transformation (tone placement, letter modification, undo on double-press) is handled entirely by the `vi` crate. Do not reimplement transformation logic in engine files.

## Verify with CLI after changes

After modifying any engine, verify manually:

```bash
cargo run -p ibus-vie-cli -- --method telex --input "vieetj"
cargo run -p ibus-vie-cli -- --method telex --trace --input "vieetj"  # trace FSM steps
```
