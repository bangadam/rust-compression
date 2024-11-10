use clap::{Arg, Command};
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::{copy, BufReader};
use std::time::Instant;

fn main() {
    // Define and parse command-line arguments using clap
    let matches = Command::new("Rust Gzip")
        .version("1.0")
        .author("Author Name <bangadam.dev@gmail.com>")
        .about("Gzip compression tool")
        .arg(
            Arg::new("source")
                .help("sets the input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("target")
                .help("sets the output file to use, recommended with `.gz` extension")
                .required(true)
                .index(2),
        )
        .get_matches();

    // Get the values of the source and target arguments
    let source = matches.get_one::<String>("source").unwrap();
    let target = matches.get_one::<String>("target").unwrap();

    // Open the source file for reading
    let input = File::open(source).expect("Failed to open file");
    let mut input = BufReader::new(input);

    // Create the target file for writing
    let output = File::create(target).expect("Failed to create file");

    // Create a GzEncoder to compress the data
    let mut encoder = GzEncoder::new(output, Compression::default());

    // Start the timer to measure compression time
    let start = Instant::now();

    // Copy data from the source file to the encoder (compressing it)
    copy(&mut input, &mut encoder).expect("Failed to compress file");

    // Ensure all data is written and the compression stream is properly closed
    encoder.finish().expect("Failed to finish compression");

    // Print the time taken for compression
    println!("Compression took: {:?}", start.elapsed());

    // Print the original file size
    println!(
        "Original file size: {}",
        input.get_ref().metadata().unwrap().len()
    );
}
