use std::env::args;

fn main() {
    if let Err(e) = fallible_main() {
        eprintln!("ERROR: {}", e);
        std::process::exit(1);
    }
}

fn fallible_main() -> Result<(), Box<dyn std::error::Error>> {
    if let Some(filename) = args().skip(1).next() {
        let markup = std::fs::read(filename)?;
        let markup = String::from_utf8_lossy(&markup);
        let mut flags = pikchr::PikchrFlags::default();
        flags.generate_plain_errors();
        let image = pikchr::Pikchr::render(&markup, None, flags)?;
        print!("{}", image);
    }
    Ok(())
}
