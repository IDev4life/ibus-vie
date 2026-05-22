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

When adding test cases, also run `cargo test --workspace` to verify they pass.
