# CloakedCanvas Architecture

This project is split into multiple pieces:

- `core/` – Rust library providing cryptographic and core utilities.
  - Currently includes a `hello()` function and a simple `xor_mask` helper
    used as a reversible placeholder until real encryption is implemented.
- `plugin/` – Adobe UXP plugin skeleton for a "Secure Export" panel.
- `web/` – WebAssembly page for decrypting protected assets.

Additional modules and details will be fleshed out in future sprints.
