extern crate nquads;

use nquads::Node::*;

#[test]
fn it_works() {
    let quads = nquads::parse("_:alice <http://xmlns.com/foaf/0.1/knows> _:bob <http://example.org/graphs/john> . #test\n_:bob <http://xmlns.com/foaf/0.1/knows> _:alice <http://example.org/graphs/james> .");

    assert_eq!(2, quads.len());

    assert_eq!(BlankNodeLabel { label: "alice".to_string() }, quads[0].subject);
    assert_eq!(IriRef { iri: "http://xmlns.com/foaf/0.1/knows".to_string() }, quads[0].predicate);
    assert_eq!(BlankNodeLabel { label: "bob".to_string() }, quads[0].object);
    assert_eq!(IriRef { iri: "http://example.org/graphs/john".to_string() }, quads[0].graph_label);
}
