use assembler::assemble_file;
use std::fs;

#[test]
fn fibbonaci() {
    let input = "examples/fib10/fib10.asm";
    let mut output = String::from("examples/fib10/fib10");
    assemble_file(input, &output);
    let actual = fs::read(output.clone()).unwrap();
    output.push_str("_expected");
    let expected = fs::read(output).unwrap();
    assert_eq!(actual, expected);
}
