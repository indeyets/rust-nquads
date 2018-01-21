extern crate nquads;

use nquads::Node::*;


#[test]
fn parse_empty_literal() {
    let literal = nquads::parse_literal("\"\"");

    let expectation = Literal {
        literal: "".to_string(),
        kind: "".to_string(),
        language: "".to_string(),
    };

    assert_eq!(expectation, literal);
}

#[test]
fn parse_literal_with_echars() {
    let literal = nquads::parse_literal("\"\\t\\b\\n\\r\\f\\\"\\'\\\\\"");

    let expectation = Literal {
        literal: "\u{9}\u{8}\u{a}\u{d}\u{c}\u{22}\u{27}\u{5c}".to_string(),
        kind: "".to_string(),
        language: "".to_string(),
    };

    assert_eq!(expectation, literal);
}
