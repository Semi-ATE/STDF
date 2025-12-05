use std::env;
use stdf::valid_file;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <stdf_file>", args[0]);
        std::process::exit(1);
    }
    
    let filename = &args[1];
    
    match valid_file(filename) {
        Ok(is_valid) => {
            if is_valid {
                println!("✓ File is valid: last record has correct length");
                std::process::exit(0);
            } else {
                println!("✗ File is invalid: last record has incorrect length");
                std::process::exit(1);
            }
        },
        Err(e) => {
            eprintln!("Error validating file: {}", e);
            std::process::exit(2);
        }
    }
}
