extern crate nquads;

use nquads::Node::*;


#[test]
fn parse_empty_literal() {
    let literal = nquads::parse_literal("\"\"");

    let expectation = Literal {
        literal: "".to_string(),
        kind: "http://www.w3.org/2001/XMLSchema#string".to_string(),
        language: None,
    };

    assert_eq!(expectation, literal);
}

#[test]
fn parse_literal_with_echars() {
    let literal = nquads::parse_literal("\"\\t\\b\\n\\r\\f\\\"\\'\\\\\"");

    let expectation = Literal {
        literal: "\u{9}\u{8}\u{a}\u{d}\u{c}\u{22}\u{27}\u{5c}".to_string(),
        kind: "http://www.w3.org/2001/XMLSchema#string".to_string(),
        language: None,
    };

    assert_eq!(expectation, literal);
}

#[test]
fn parse_literal_with_language() {
    let literal = nquads::parse_literal("\"\"^^@en-US");

    let expectation = Literal {
        literal: "".to_string(),
        kind: "http://www.w3.org/1999/02/22-rdf-syntax-ns#langString".to_string(),
        language: Some("en-US".to_string()),
    };

    assert_eq!(expectation, literal);
}

#[test]
fn parse_literal_with_type() {
    let literal = nquads::parse_literal("\"\"^^<http://example.org/foo>");

    let expectation = Literal {
        literal: "".to_string(),
        kind: "http://example.org/foo".to_string(),
        language: None,
    };

    assert_eq!(expectation, literal);
}
