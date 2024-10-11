use std::{env, process};

mod compressor;
mod decompressor;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <compress|decompress> <input_file> <output_file>", args[0]);
        process::exit(1);
    }

    let command = &args[1];
    let input_file = &args[2];
    let output_file = &args[3];

    match command.as_str() {
        "compress" => {
            match compressor::compress(input_file, output_file) {
                Ok(_) => println!("File compressed successfully."),
                Err(e) => eprintln!("Error compressing file: {}", e),
            }
        }
        "decompress" => {
            match decompressor::decompress(input_file, output_file) {
                Ok(_) => println!("File decompressed successfully."),
                Err(e) => eprintln!("Error decompressing file: {}", e),
            }
        }
        _ => {
            eprintln!("Invalid command. Use 'compress' or 'decompress'.");
            process::exit(1);
        }
    }
}
