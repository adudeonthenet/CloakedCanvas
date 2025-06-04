# CloakedCanvas Workflows

## Protecting Images (Level 2 Default)
1. Designer selects layers in Photoshop and clicks **Export Selected** in the
   CloakedCanvas panel.
2. The plugin packages a flattened copy and sends it to the core WASM module
   for encryption.
3. The encrypted `.ccvault` file is saved alongside a low‑res preview image
   watermarked with `assets/branding/preview_watermark.svg`.

## Protecting Documents
Document protection follows the same flow but can operate on PDFs or other
binary assets outside of Photoshop using the command‑line version of the core
library.
