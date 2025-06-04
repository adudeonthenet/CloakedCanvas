use clap::{Parser, Subcommand};
use cloakedcanvas_core::{
    default_watermark_path, encrypt_docx_to_vault, encrypt_file, generate_preview_img,
    rasterize_preview_from_pdf, KEY_SIZE,
};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Encrypt file to .ccvault
    Encrypt { path: PathBuf },
    /// Generate preview image
    Preview { path: PathBuf },
    /// Protect document (DOCX or PDF)
    ProtectDoc { path: PathBuf },
}

fn main() {
    let cli = Cli::parse();
    let key = [42u8; KEY_SIZE];
    match cli.command {
        Commands::Encrypt { path } => match encrypt_file(&path, &key) {
            Ok(out) => println!("Encrypted to {}", out.display()),
            Err(e) => eprintln!("Error: {e}"),
        },
        Commands::Preview { path } => {
            match generate_preview_img(&path, &cloakedcanvas_core::default_watermark_path()) {
                Ok(out) => println!("Preview saved to {}", out.display()),
                Err(e) => eprintln!("Error: {e}"),
            }
        }
        Commands::ProtectDoc { path } => {
            let enc_res = if path.extension().map(|e| e == "docx").unwrap_or(false) {
                encrypt_docx_to_vault(&path, &key)
            } else {
                encrypt_file(&path, &key)
            };
            match enc_res {
                Ok(vault) => {
                    println!("Encrypted to {}", vault.display());
                    let preview_res = if path.extension().map(|e| e == "pdf").unwrap_or(false) {
                        rasterize_preview_from_pdf(&path, &default_watermark_path())
                    } else {
                        generate_preview_img(&path, &default_watermark_path())
                    };
                    if let Ok(preview) = preview_res {
                        println!("Preview saved to {}", preview.display());
                    }
                }
                Err(e) => eprintln!("Error: {e}"),
            }
        }
    }
}
