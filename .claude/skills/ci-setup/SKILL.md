---
name: ci-setup
description: Scaffold GitHub Actions CI workflow for this Rust workspace. Creates .github/workflows/ci.yml with fmt, clippy, test, and build jobs.
---

Create `.github/workflows/ci.yml` with the following jobs, using the stable toolchain from `rust-toolchain.toml`:

1. **fmt** — `cargo fmt --all -- --check`
2. **clippy** — `cargo clippy --all-targets --all-features -- -D warnings`
3. **test** — `cargo test --workspace`
4. **build** — `cargo build --release --workspace`

Trigger on: push to main, pull_request to main.

Use `actions/checkout@v4` and `dtolnay/rust-toolchain@stable`.
Cache cargo registry and target dir with `Swatinem/rust-cache@v2`.

After creating the file, report the path and summary of jobs.
