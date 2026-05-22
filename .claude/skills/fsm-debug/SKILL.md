---
name: fsm-debug
description: Run the ibus-vie FSM debug CLI to test input method output without needing IBus running. Args: [method] [input]. Method: telex|vni|viqr (default: telex).
---

Parse args from the user's message:
- method: one of telex, vni, viqr (default: telex)
- input: string to test (optional)

If input provided, run:
```
cargo run -p ibus-vie-cli -- --method {method} --input "{input}"
```

If no input, run interactively:
```
cargo run -p ibus-vie-cli -- --method {method}
```

Report the output directly. If cargo fails to compile, show the error and suggest `cargo build -p ibus-vie-cli` to diagnose.
