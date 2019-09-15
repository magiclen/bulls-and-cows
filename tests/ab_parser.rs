extern crate bulls_and_cows;

use bulls_and_cows::parser::ABParser;

#[test]
fn parse_xayb() {
    let ab = ABParser::new();

    assert_eq!((4, 0), ab.parse("4A0B").unwrap());
    assert_eq!((2, 1), ab.parse("2a1B").unwrap());
    assert_eq!((0, 4), ab.parse("0a4b").unwrap());
}
