# CloakedCanvas Admin Guide

## Building Components
- **Core Library** – Run `cargo build --manifest-path core/Cargo.toml` to compile the Rust code. Use `wasm-pack build core --out-dir extension/wasm --release` when updating the browser extension.
- **Web Decrypt Page** – Execute `npm install --prefix web` once, then `npm run --prefix web build` to regenerate `web/pkg/`.
- **Browser Extension** – Load the `extension/` directory as an unpacked extension in Chrome or Firefox after building the WASM module.

## Deploying the Plugin
Distribute the `plugin/` folder through the Adobe UXP Developer Tool or package it with `uxp pack`. Ensure designers have access to the storage provider URLs configured in their environment.

## Storage Configuration
The core library includes adapters for Local Disk, S3, Google Drive and Dropbox. Edit `core/src/store.rs` to customise endpoints or add new providers. Local Disk starts a small HTTP server so previews can be shared over `http://localhost:<port>`.

## License Heartbeat
Call `license_heartbeat()` from `core` on startup to obtain a 7‑day JWT. This requires mTLS credentials and a shared secret matching the server configuration.

## Key Management
Use the Shamir helpers in `core::shamir` to split the master key into three shares. Any two shares can reconstruct the key:
```rust
let shares = split_key(&master_key);
let recovered = combine_key(&shares[0..2]).unwrap();
```
Store shares separately for recovery purposes.

## Updating Nightshade
`core::nightshade` provides a placeholder poisoning algorithm. Replace `poison()` and `unpoison()` with your implementation if higher‑level protection is required.

## Hosting the Decrypt Page
Serve the contents of the `web/` directory as a static site or behind authentication as needed. Users drag `.ccvault` files onto the page to view protected assets.
