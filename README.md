# CloakedCanvas Scaffold

This repository scaffold was generated on 2025-05-15.

It includes:
- `core/` – Rust crypto/core library
- `plugin/` – Adobe UXP Secure Export panel
- `web/` – WASM decrypt page
- `docs/` – initial architecture & workflow docs
- `.github/` – CI pipeline

The code is gradually taking shape. The `core` crate now provides basic
AES‑GCM encryption utilities used by both the Photoshop plugin and the web
decrypt page. Run `cargo test --manifest-path core/Cargo.toml` to execute the
unit tests.



Join the Discord! https://discord.gg/hkCb5GzMq2
