extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::char;
use std::u32;
use pest::iterators::Pair;
use pest::Parser;

mod errors;
use errors::ParseError;

mod grammar;
use grammar::*;

trait NQuadsString {
    fn from_pair(pair: Pair<Rule>) -> String;
}

impl NQuadsString for String {
    fn from_pair(pair: Pair<Rule>) -> String {
        let mut result = String::new();

        for piece in pair.into_inner() {
            match piece.as_rule() {
                Rule::uchar4 => {
                    let chars = piece.clone().into_span().as_str();
                    let num = u32::from_str_radix(chars, 16);
                    let c = char::from_u32(num.unwrap());

                    result.push(c.unwrap());
                },
                Rule::uchar8 => {
                    let chars = piece.clone().into_span().as_str();
                    let num = u32::from_str_radix(chars, 16);
                    let c = char::from_u32(num.unwrap());

                    result.push(c.unwrap());
                },
                Rule::echar_char => {
                    let s = match piece.into_span().as_str() {
                        "t" => { "\t" },
                        "b" => { "\u{8}" },
                        "n" => { "\n" },
                        "r" => { "\r" },
                        "f" => { "\u{c}" },
                        "\"" => { "\"" },
                        "\'" => { "\'" },
                        "\\" => { "\\" },
                        _ => unreachable!()
                    };

                    result += s;
                },
                _ => {
                    result += piece.clone().into_span().as_str();
                }
            };
        }

        result
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Node {
    BlankNodeLabel { label: String },
    IriRef { iri: String },
    Literal { literal: String, kind: String, language: Option<String> },
}

const RDF_LANG_STRING: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#langString";
const XSD_STRING: &str = "http://www.w3.org/2001/XMLSchema#string";

impl Node {
    fn from_blank_node_label(pair: Pair<Rule>) -> Node {
        Node::BlankNodeLabel { label: String::from_pair(pair) }
    }

    fn from_iriref(pair: Pair<Rule>) -> Node {
        Node::IriRef { iri: String::from_pair(pair) }
    }

    fn from_literal(pair: Pair<Rule>) -> Node {
        let mut pieces = pair.into_inner();

        let literal = String::from_pair(pieces.next().unwrap());

        match pieces.next() {
            Some(piece2) => {
                match piece2.as_rule() {
                    Rule::iriref => Node::Literal {
                        literal,
                        kind: String::from_pair(piece2),
                        language: None
                    },
                    Rule::langtag => Node::Literal {
                        literal,
                        kind: RDF_LANG_STRING.to_owned(),
                        language: Some(piece2.clone().into_span().as_str().to_owned())
                    },
                    _ => unreachable!()
                }
            },
            None => {
                Node::Literal {
                    literal,
                    kind: XSD_STRING.to_owned(),
                    language: None
                }
            }
        }
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Quad {
    pub subject:     Node,
    pub predicate:   Node,
    pub object:      Node,
    pub graph_label: Option<Node>,
}

impl Quad {
    fn from_statement(pair: Pair<Rule>) -> Quad {
        let mut pieces = pair.into_inner();

        let _subject = pieces.next().unwrap();

        let __subject = _subject.into_inner().next().unwrap();
        let subject = match __subject.as_rule() {
            Rule::iriref => Node::from_iriref(__subject),
            Rule::blank_node_label => Node::from_blank_node_label(__subject),
            _ => unreachable!()
        };

        let _predicate = pieces.next().unwrap();
        let __predicate = _predicate.into_inner().next().unwrap();
        let predicate = match __predicate.as_rule() {
            Rule::iriref => Node::from_iriref(__predicate),
            _ => unreachable!()
        };

        let _object = pieces.next().unwrap();
        let __object = _object.into_inner().next().unwrap();
        let object = match __object.as_rule() {
            Rule::iriref => Node::from_iriref(__object),
            Rule::blank_node_label => Node::from_blank_node_label(__object),
            Rule::literal => Node::from_literal(__object),
            _ => unreachable!()
        };

        let _graph = pieces.next();

        let graph_label = match _graph {
            None => None,
            _ => Some({
                let __graph = _graph.unwrap().into_inner().next().unwrap();
                match __graph.as_rule() {
                    Rule::iriref => Node::from_iriref(__graph),
                    Rule::blank_node_label => Node::from_blank_node_label(__graph),
                    _ => unreachable!()
                }
            })
        };

        Quad {
            subject,
            predicate,
            object,
            graph_label
        }
    }
}

pub fn parse(input: &str) -> Result<Vec<Quad>, ParseError> {
    match NQuadsParser::parse(Rule::expression, input) {
        Ok(pairs) => {
            let mut vec = Vec::new();

            for pair in pairs {
                vec.push(Quad::from_statement(pair))
            }

            Ok(vec)
        },
        Err(e) => {
            Err(errors::ParseError::Pest(e))
        }
    }

}

pub fn parse_iriref(input: &str) -> Result<Node, ParseError> {
    match NQuadsParser::parse(Rule::_iriref, input) {
        Ok(pairs) => {
            match pairs.clone().next() {
                Some(literal) => Ok(Node::from_iriref(literal)),
                None => Err(errors::ParseError::EmptyInput)
            }
        },
        Err(e) => {
            Err(errors::ParseError::Pest(e))
        }
    }
}

pub fn parse_literal(input: &str) -> Result<Node, ParseError> {
    match NQuadsParser::parse(Rule::literal, input) {
        Ok(pairs) => {
            match pairs.clone().next() {
                Some(literal) => Ok(Node::from_literal(literal)),
                None => Err(errors::ParseError::EmptyInput)
            }
        },
        Err(e) => {
            Err(errors::ParseError::Pest(e))
        }
    }
}
