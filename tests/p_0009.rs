use rust_aoj::io::value_reader::StringReader;
use rust_aoj::io::value_writer::StringWriter;
use rust_aoj::problems::p_0009::solve;

#[test]
fn p_0009() {
    let input = "10\n3\n11\n100\n\n".as_bytes();
    let mut output = Vec::<u8>::new();
    let reader = StringReader::new(input);
    let writer = StringWriter::new(&mut output);

    solve(reader, writer);

    let expected = "4\n2\n5\n25\n";

    assert_eq!(String::from_utf8(output).unwrap(), expected)
}
