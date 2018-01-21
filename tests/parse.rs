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
        graph_label:    IriRef { iri: "http://example.org/graphs/john".to_string() },
    };

    assert_eq!(expectation_0, quads[0]);

    let expectation_1 = Quad {
        subject:        BlankNodeLabel { label: "bob".to_string() },
        predicate:      IriRef { iri: "http://xmlns.com/foaf/0.1/knows".to_string() },
        object:         BlankNodeLabel { label: "alice".to_string() },
        graph_label:    IriRef { iri: "http://example.org/graphs/james".to_string() },
    };

    assert_eq!(expectation_1, quads[1]);
}

#[test]
fn parse_iri_with_uchars() {
    let quads = nquads::parse("_:1 <http://example.com/\\u0041\\U00000042> _:2 <http://example.com> .");

    assert_eq!(1, quads.len());

    let expectation_0 = Quad {
        subject:        BlankNodeLabel { label: "1".to_string() },
        predicate:      IriRef { iri: "http://example.com/AB".to_string() },
        object:         BlankNodeLabel { label: "2".to_string() },
        graph_label:    IriRef { iri: "http://example.com".to_string() },
    };

    assert_eq!(expectation_0, quads[0]);
}

#[test]
fn parse_simple_literal() {
    let quads = nquads::parse("_:1 <http://example.com/foo> \"\" <http://example.com> .");
    assert_eq!(1, quads.len());

    let expectation = Quad {
        subject:        BlankNodeLabel { label: "1".to_string() },
        predicate:      IriRef { iri: "http://example.com/foo".to_string() },
        object: Literal {
            literal: "".to_string(),
            kind: "".to_string(),
            language: "".to_string(),
        },
        graph_label:    IriRef { iri: "http://example.com".to_string() },
    };

    assert_eq!(expectation, quads[0]);
}

#[test]
fn parse_literal_with_echars() {
    let quads = nquads::parse("_:1 <http://example.com/foo> \"\\t\\b\\n\\r\\f\\\"\\'\\\\\" <http://example.com> .");
    assert_eq!(1, quads.len());

    let expectation = Quad {
        subject:        BlankNodeLabel { label: "1".to_string() },
        predicate:      IriRef { iri: "http://example.com/foo".to_string() },
        object: Literal {
            literal: "\u{9}\u{8}\u{a}\u{d}\u{c}\u{22}\u{27}\u{5c}".to_string(),
            kind: "".to_string(),
            language: "".to_string(),
        },
        graph_label:    IriRef { iri: "http://example.com".to_string() },
    };

    assert_eq!(expectation, quads[0]);
}
