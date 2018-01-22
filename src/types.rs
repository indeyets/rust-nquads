#[derive(Debug)]
#[derive(PartialEq)]
pub enum Node {
    BlankNodeLabel { label: String },
    IriRef { iri: String },
    Literal { literal: String, kind: String, language: Option<String> },
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Quad {
    pub subject:     Node,
    pub predicate:   Node,
    pub object:      Node,
    pub graph_label: Option<Node>,
}
