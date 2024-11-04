mod parser;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Too few arguments passed in");
        std::process::exit(1);
    }
    let input_filename = &args[1];
    let output_filename = &args[2];
    assembler::assemble_file(input_filename, output_filename);
}
