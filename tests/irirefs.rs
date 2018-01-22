extern crate nquads;

use nquads::Node::*;


#[test]
fn parse_iri_with_uchars() {
    let result = nquads::parse_iriref("<http://example.com/\\u0041\\U00000042>");
    assert!(result.is_ok());

    let iriref = result.unwrap();
    let expectation = IriRef { iri: "http://example.com/AB".to_string() };

    assert_eq!(expectation, iriref);
}
