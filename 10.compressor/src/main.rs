extern crate flate2;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::env;
use std::fs::File;
use std::io::{self, BufReader, copy};
use std::path::PathBuf;
use std::time::Instant;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <source> <target>", args[0]);
        return Ok(());
    }

    let source_path: PathBuf = PathBuf::from(&args[1]);
    let target_path: PathBuf = PathBuf::from(&args[2]);

    // Get the file
    let file = File::open(&source_path).expect("[!] Could not read the file!");
    let mut input = BufReader::new(file);

    // Output file
    let output = File::create(&target_path).expect("[!] Could not create the target file!");

    // Compress the file
    let mut encoder = GzEncoder::new(output, Compression::default());
    let start = Instant::now();

    copy(&mut input, &mut encoder)?;

    let output = encoder.finish()?;

    // Std outputs
    println!(
        "[*] Source length: {} bytes",
        input.get_ref().metadata()?.len()
    );
    println!(
        "[*] Target length: {} bytes",
        output.metadata()?.len()
    );
    println!(
        "[*] Elapsed time: {:?}",
        start.elapsed()
    );

    Ok(())
}
