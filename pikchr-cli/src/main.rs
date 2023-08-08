use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Clone, Parser)]
#[command(author, version, about, long_about=None)]
struct Args {
    /// Pikchr file to convert to SVG
    pikchr: PathBuf,
}

fn main() {
    let args = Args::parse();
    if let Err(e) = fallible_main(&args) {
        eprintln!("ERROR: {}", e);
        std::process::exit(1);
    }
}

fn fallible_main(args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    let markup = std::fs::read(&args.pikchr)?;
    let markup = String::from_utf8_lossy(&markup);
    let mut flags = pikchr::PikchrFlags::default();
    flags.generate_plain_errors();
    let image = pikchr::Pikchr::render(&markup, None, flags)?;
    print!("{}", image);
    Ok(())
}
