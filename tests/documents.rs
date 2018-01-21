extern crate nquads;

use nquads::Node::*;
use nquads::Quad;

#[test]
fn parse_2_statements() {
    let quads = nquads::parse("_:alice <http://xmlns.com/foaf/0.1/knows> _:bob <http://example.org/graphs/john> . #test\n_:bob <http://xmlns.com/foaf/0.1/knows> _:alice <http://example.org/graphs/james> .");

    assert_eq!(2, quads.len());

    let expectation_0 = Quad {
        subject:        BlankNodeLabel { label: "alice".to_string() },
        predicate:      IriRef { iri: "http://xmlns.com/foaf/0.1/knows".to_string() },
        object:         BlankNodeLabel { label: "bob".to_string() },
        graph_label:    Some(IriRef { iri: "http://example.org/graphs/john".to_string() }),
    };

    assert_eq!(expectation_0, quads[0]);

    let expectation_1 = Quad {
        subject:        BlankNodeLabel { label: "bob".to_string() },
        predicate:      IriRef { iri: "http://xmlns.com/foaf/0.1/knows".to_string() },
        object:         BlankNodeLabel { label: "alice".to_string() },
        graph_label:    Some(IriRef { iri: "http://example.org/graphs/james".to_string() }),
    };

    assert_eq!(expectation_1, quads[1]);
}

#[test]
fn parse_statement_with_default_graph() {
    let quads = nquads::parse("_:alice <http://xmlns.com/foaf/0.1/knows> _:bob .");

    assert_eq!(1, quads.len());

    let expectation_0 = Quad {
        subject:        BlankNodeLabel { label: "alice".to_string() },
        predicate:      IriRef { iri: "http://xmlns.com/foaf/0.1/knows".to_string() },
        object:         BlankNodeLabel { label: "bob".to_string() },
        graph_label:    None,
    };

    assert_eq!(expectation_0, quads[0]);
}
