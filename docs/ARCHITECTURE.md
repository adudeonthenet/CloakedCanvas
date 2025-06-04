# CloakedCanvas Architecture

The project consists of three primary pieces:

1. **Core (Rust)** – provides encryption utilities compiled for both native
   and WebAssembly targets. Currently implements AES-GCM with a simple API.
2. **Photoshop Plugin** – UXP panel that invokes the core library (via WASM)
   to securely export selected layers or documents.
3. **Web Decrypt Page** – standalone WASM page that decrypts `.ccvault`
   files for viewing when shared with collaborators.

This repository is early stage but now includes working Rust tests for the
encryption module.
