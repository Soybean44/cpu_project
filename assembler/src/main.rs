mod assembler;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Too few arguments passed in");
        std::process::exit(1);
    }
    let filename = &args[1];
    let contents = fs::read_to_string(filename);
    if !contents.is_ok() {
        eprintln!("File does not exist.");
        std::process::exit(1);
    }
    let contents = contents.unwrap();
    dbg!(assembler::assemble(&contents));
}
