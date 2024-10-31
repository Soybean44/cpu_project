mod assembler;

use std::fs;
use std::{mem, slice};

fn assemble_file(input_filename: &str, output_filename: &str) {
    let contents = fs::read_to_string(input_filename);
    if !contents.is_ok() {
        eprintln!("File does not exist.");
        std::process::exit(1);
    }
    let contents = contents.unwrap();
    let machine_code = assembler::assemble(&contents);
    let machine_code_u8: &[u8] = unsafe {
        slice::from_raw_parts(
            machine_code.as_ptr() as *const u8,
            machine_code.len() * mem::size_of::<u16>(),
        )
    };

    fs::write(output_filename, machine_code_u8).expect("Could not write to file");
}
