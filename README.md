# CloakedCanvas Scaffold

This repository scaffold was generated on 2025-05-15.

It includes:
- `core/` – Rust crypto/core library
- `plugin/` – Adobe UXP Secure Export panel
- `web/` – WASM decrypt page
- `docs/` – initial architecture & workflow docs
- `.github/` – CI pipeline

The project now includes preview generation, vault encryption and a set of
storage adapters for Local, S3, Google Drive and Dropbox. A stub implementation
of the Nightshade poisoning algorithm is available along with a 2-of-3 Shamir
key escrow helper. A license heartbeat can be performed over mTLS to obtain a
7‑day JWT.

Run `cargo test --manifest-path core/Cargo.toml` to execute the unit tests and
`npm install --prefix web && npm run --prefix web build` to build the web
decrypt page.



Join the Discord! https://discord.gg/hkCb5GzMq2
