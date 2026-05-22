---
description: Vietnamese tone placement algorithm
globs: crates/ibus-vie-vi/src/tone.rs
---

# Tone Placement Rules (kiểu mới — default)

The `place_tone` function follows this priority:

1. **Single vowel** → tone on it
2. **Exactly one marked vowel** (ê, ô, ơ, ư, â, ă) → tone on the marked vowel
3. **3+ vowels** → tone on the middle one (e.g., "ươi" → ơ)
4. **2 vowels with coda consonant** → tone on the second (last) vowel
5. **2 vowels without coda** → tone on the first vowel

Examples:

- "ba" + sắc → "bá" (rule 1)
- "viêt" + nặng → "việt" (rule 2: ê is marked)
- "ngươi" + huyền → "người" (rule 3: middle of ư-ơ-i)
- "hoang" + huyền → "hoàng" (rule 4: second vowel, has coda)
- "chao" + huyền → "chào" (rule 5: first vowel, no coda)
- "hoa" + huyền → "hòa" (rule 5: first vowel, no coda)

When modifying this file, use the `linguistic-reviewer` agent to validate changes.
