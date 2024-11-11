use assembler::assemble_file;
use std::fs;

#[test]
fn increment() {
    let input = "examples/inc/inc.asm";
    let mut output = String::from("examples/inc/inc");
    assemble_file(input, &output);
    let actual = fs::read(output.clone()).unwrap();
    output.push_str("_expected");
    let expected = fs::read(output).unwrap();
    assert_eq!(actual, expected);
}
