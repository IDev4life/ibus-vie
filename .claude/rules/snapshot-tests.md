---
description: Snapshot test format and conventions
globs: tests/snapshot/**/*.txt
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

## Current test setup

Tests live as inline unit tests in each engine file (`telex.rs`, `vni.rs`, `viqr.rs`) using `engine.feed_str(input)`. The snapshot `.txt` files document expected behavior and serve as the source-of-truth reference.

When adding new cases:

1. Add a line to the corresponding `tests/snapshot/<method>.txt`
2. Add a matching `#[test]` in the engine's `mod tests` block
3. Run `cargo test --workspace` to verify
