use assembler::assemble_file;
use std::fs;

#[test]
fn addition() {
    let input = "examples/add/add.asm";
    let mut output = String::from("examples/add/add");
    assemble_file(input, &output);
    let actual = fs::read(output.clone()).unwrap();
    output.push_str("_expected");
    let expected = fs::read(output).unwrap();
    assert_eq!(actual, expected);
}
