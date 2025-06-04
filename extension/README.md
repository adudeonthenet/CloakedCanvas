# CloakedCanvas Browser Extension

This extension intercepts file uploads to protect them with a preview and vault
stored in user-owned storage. It also detects `*.ccvault` links on pages and can
decrypt them on demand.

## Development

1. Build the WASM module:
   ```bash
   wasm-pack build core --out-dir extension/wasm --release
   ```
2. Load the `extension/` directory as an unpacked extension in Chrome or
   Firefox (use `about:debugging`).

Firefox requires the `browser.*` API polyfill and manifest v2. The files in this
folder work for Manifest V3 browsers.

The repository does not include the `cloakedcanvas_wasm_bg.wasm` binary. Running
the build command above will generate it before packaging the extension.

The extension icons are not included to avoid binary diffs. Add 48x48 and 128x128 PNG files under `icons/` before packaging.
