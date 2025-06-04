/*
 * CloakedCanvas - MIT License
 */

use crate::{encrypt_data, KEY_SIZE};
use image::{DynamicImage, GenericImageView, GenericImage, RgbaImage};
use std::path::{Path, PathBuf};
use std::fs;

use usvg::Options;
use tiny_skia::Pixmap;
use resvg::render;
use std::process::Command;

/// Path to default watermark SVG bundled with the crate.
pub fn default_watermark_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../assets/branding/preview_watermark.svg")
}

/// Encrypt an arbitrary file into a `.ccvault` file.
pub fn encrypt_file(input_path: &Path, key: &[u8; KEY_SIZE]) -> Result<PathBuf, String> {
    let data = fs::read(input_path).map_err(|e| e.to_string())?;
    let (ct, nonce) = encrypt_data(key, &data)?;
    let out_path = input_path.with_extension("ccvault");
    let mut out_data = Vec::new();
    out_data.extend(&nonce);
    out_data.extend(&ct);
    fs::write(&out_path, out_data).map_err(|e| e.to_string())?;
    Ok(out_path)
}

/// Generate a downsampled preview PNG with watermark.
pub fn generate_preview_img(input_path: &Path, watermark_svg: &Path) -> Result<PathBuf, String> {
    let mut img = image::open(input_path).map_err(|e| e.to_string())?;
    let (w, h) = img.dimensions();
    let max_dim = w.max(h);
    let scale = if max_dim > 1000 { 1000.0 / max_dim as f32 } else { 1.0 };
    let new_w = (w as f32 * scale) as u32;
    let new_h = (h as f32 * scale) as u32;
    img = img.resize(new_w, new_h, image::imageops::FilterType::CatmullRom);

    let watermark = render_svg_to_image(watermark_svg)?;
    overlay_center(&mut img, &watermark, 0.15);

    let out = input_path.with_file_name("preview.png");
    img.save(&out).map_err(|e| e.to_string())?;
    Ok(out)
}

fn render_svg_to_image(path: &Path) -> Result<DynamicImage, String> {
    let svg_data = fs::read(path).map_err(|e| e.to_string())?;
    let opt = Options::default();
    let rtree = usvg::Tree::from_data(&svg_data, &opt).map_err(|e| e.to_string())?;
    let pixmap_size = rtree.size().to_int_size();
    let mut pixmap = Pixmap::new(pixmap_size.width(), pixmap_size.height()).ok_or("pixmap")?;
    render(&rtree, tiny_skia::Transform::default(), &mut pixmap.as_mut());
    let img = RgbaImage::from_raw(pixmap.width(), pixmap.height(), pixmap.take()).ok_or("raw")?;
    Ok(DynamicImage::ImageRgba8(img))
}

fn overlay_center(base: &mut DynamicImage, overlay: &DynamicImage, opacity: f32) {
    let (bw, bh) = base.dimensions();
    let (ow, oh) = overlay.dimensions();
    let x = (bw.saturating_sub(ow)) / 2;
    let y = (bh.saturating_sub(oh)) / 2;
    for oy in 0..oh {
        for ox in 0..ow {
            let p = overlay.get_pixel(ox, oy);
            let mut b = base.get_pixel(x + ox, y + oy);
            let alpha = (p[3] as f32 / 255.0) * opacity;
            for i in 0..3 {
                b[i] = (b[i] as f32 * (1.0 - alpha) + p[i] as f32 * alpha) as u8;
            }
            base.put_pixel(x + ox, y + oy, b);
        }
    }
}

/// Convert a DOCX file to PDF using LibreOffice, then encrypt to `.ccvault`.
pub fn encrypt_docx_to_vault(path: &Path, key: &[u8; KEY_SIZE]) -> Result<PathBuf, String> {
    let out_dir = path.parent().ok_or("no parent")?;
    let status = Command::new("libreoffice")
        .args(["--headless", "--convert-to", "pdf", path.to_str().unwrap(), "--outdir", out_dir.to_str().unwrap()])
        .status()
        .map_err(|e| e.to_string())?;
    if !status.success() {
        return Err("LibreOffice conversion failed".into());
    }
    let pdf_path = path.with_extension("pdf");
    encrypt_file(&pdf_path, key)
}

/// Rasterize PDF to PNG using Ghostscript.
pub fn rasterize_preview_from_pdf(path: &Path, watermark: &Path) -> Result<PathBuf, String> {
    let out = path.with_file_name("preview.png");
    let status = Command::new("gs")
        .args(["-sDEVICE=png16m", "-o", out.to_str().unwrap(), "-r72", path.to_str().unwrap()])
        .status()
        .map_err(|e| e.to_string())?;
    if !status.success() {
        return Err("Ghostscript failed".into());
    }
    generate_preview_img(&out, watermark)
}
