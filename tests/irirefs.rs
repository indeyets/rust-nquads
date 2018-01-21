extern crate nquads;

use nquads::Node::*;


#[test]
fn parse_iri_with_uchars() {
    let iriref = nquads::parse_iriref("<http://example.com/\\u0041\\U00000042>");
    let expectation = IriRef { iri: "http://example.com/AB".to_string() };

    assert_eq!(expectation, iriref);
}
