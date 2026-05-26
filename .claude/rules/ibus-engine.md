---
description: Rules for IBus DBus integration code
globs: crates/ibus-vie-engine/**/*.rs
---

# IBus Engine Rules

## All IBus/DBus code stays here

`ibus-vie-engine` is the ONLY crate that touches IBus, DBus, or any system I/O. Never leak IBus concepts into `ibus-vie-im`.

## Key translation

`ibus/engine/mod.rs` translates IBus keyval/state → `KeyEvent`:

- Ignore key release events (bit 30 of state)
- Ignore keys with Ctrl or Alt modifiers (commits pending preedit, then passes through)
- Map `0xff08` → Backspace, `0xff1b` → Escape
- `0xff0d` (Enter) → commits pending preedit directly (no KeyEvent created)
- Navigation keys (`0xff51..0xff57`, `0xff50`) → commit preedit, pass through
- Map `0x20..=0x7e` → printable ASCII char

## Engine lifecycle

- `ibus-daemon` spawns the binary and calls `CreateEngine` on the factory
- Each engine instance wraps a `Box<dyn Engine>` selected by active method from config/property menu:
  - `"telex"` → `TelexEngine`
  - `"vni"` → `VniEngine`
- On `focus_out`, commit any pending preedit and reset
- On `disable`, reset without committing
- On `enable`, register IBus properties (method menu, mode menu, version label)
- Property menu allows switching Telex/VNI and preedit/popup at runtime
