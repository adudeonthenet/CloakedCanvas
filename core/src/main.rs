use clap::{Parser, Subcommand};
use cloakedcanvas_core::{encrypt_file, generate_preview_img, KEY_SIZE};
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
}

fn main() {
    let cli = Cli::parse();
    let key = [42u8; KEY_SIZE];
    match cli.command {
        Commands::Encrypt { path } => {
            match encrypt_file(&path, &key) {
                Ok(out) => println!("Encrypted to {}", out.display()),
                Err(e) => eprintln!("Error: {e}"),
            }
        }
        Commands::Preview { path } => {
            match generate_preview_img(&path, &cloakedcanvas_core::default_watermark_path()) {
                Ok(out) => println!("Preview saved to {}", out.display()),
                Err(e) => eprintln!("Error: {e}"),
            }
        }
    }
}
