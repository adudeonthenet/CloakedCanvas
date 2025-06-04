# CloakedCanvas User Guide

## Installing the Plugin
1. Open Adobe UXP Developer Tool.
2. Choose **Load** and select the `plugin/` folder of this repository.
3. The panel will appear under **Plugins > CloakedCanvas Secure Export**.

## Protecting Images
1. Open your document and select the layers you wish to export.
2. In the Secure Export panel choose a protection level:
   - **Protected Preview (L2)** – exports a watermarked preview alongside a vault file.
   - **Editable + Nightshade (L3)** – applies the Nightshade algorithm before encryption.
3. Pick a storage provider from the dropdown (Local Disk, S3, Google Drive or Dropbox).
4. Click **Export & Copy Markdown**. A `.ccvault` file and `preview.png` are created and uploaded using the selected adapter. A Markdown snippet referencing the preview is copied to your clipboard for easy sharing.

## Decrypting Assets
- Drag and drop a `.ccvault` file onto `web/index.html` to view the decrypted image in your browser.
- Alternatively install the browser extension under `extension/` to automatically detect and decrypt links ending in `.ccvault`.

## Command Line Usage
The core library exposes a CLI for batch operations:
```bash
cargo run --manifest-path core/Cargo.toml -- preview /path/to/image.png
```
Replace `preview` with `encrypt` or `protect-doc` to create vaults from other file types.
