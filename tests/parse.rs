extern crate nquads;

#[test]
fn it_works() {
    let quads = nquads::parse("_:alice <http://xmlns.com/foaf/0.1/knows> _:bob <http://example.org/graphs/john> . #test\n_:bob <http://xmlns.com/foaf/0.1/knows> _:alice <http://example.org/graphs/james> .");

    assert_eq!(2, quads.len());

    assert_eq!("alice", quads[0].subject);
    assert_eq!("http://xmlns.com/foaf/0.1/knows", quads[0].predicate);
    assert_eq!("bob", quads[0].object);
    assert_eq!("http://example.org/graphs/john", quads[0].graph_label);
}
