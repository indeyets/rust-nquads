#![recursion_limit="128"]
#[macro_use]
extern crate pest;

use pest::prelude::*;
use std::collections::LinkedList;


#[derive(Debug)]
#[derive(PartialEq)]
pub enum Node {
    BlankNodeLabel { label: String },
    IriRef { iri: String },
    Literal { literal: String, kind: String, language: String },
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Quad {
    pub subject:     Node,
    pub predicate:   Node,
    pub object:      Node,
    pub graph_label: Node,
}

impl_rdp! {
    grammar! {
        expression = _{ statement? ~ (eol ~ statement)* ~ eol? }
        statement =   { subject ~ predicate ~ object ~ graph_label? ~ ["."] }

        subject =     @{ _iriref | _blank_node_label }
        predicate =   @{ _iriref }
        object =      @{ _iriref | _blank_node_label | literal }
        graph_label = @{ _iriref | _blank_node_label }

        _blank_node_label     = _{ ["_:"] ~ blank_node_label }
        _iriref               = _{ ["<"] ~ iriref ~ [">"] }
        _langtag              = _{ ["@"] ~ langtag }
        _string_literal_quote = _{ ["\""] ~ string_literal_quote ~ ["\""] }

        blank_node_label =     { (pn_chars_u | ascii_digits) ~ pn_chars? ~ (["."]* ~ pn_chars)* }
        iriref =               { iriref_symbol* }
        literal =              { _string_literal_quote ~ (["^^"] ~ (_iriref | _langtag))? }

        langtag =              { ascii_alpha+ ~ (["-"] ~ (ascii_alpha | ascii_digits))* }
        string_literal_quote = { (echar | uchar | (!(["\u{22}"] | ["\u{5c}"] | ["\u{a}"] | ["\u{d}"]) ~ any))* }

        eol =           _{ (["\n"] | ["\r"])+ }
        iriref_symbol = _{ uchar | (!(['\u{00}'..'\u{20}'] | ["<"] | [">"] | ["\""] | ["{"] | ["}"] | ["|"] | ["^"] | ["`"] | ["\\"]) ~ any) }
        uchar =         _{ (["\\u"] ~ hex ~ hex ~ hex ~ hex) | (["\\U"] ~ hex ~ hex ~ hex ~ hex ~ hex ~ hex ~ hex ~ hex) }
        echar =         _{ ["\\"] ~ (["t"] | ["b"] | ["n"] | ["r"] | ["f"] | ["\""] | ["\'"] | ["\\"]) }
        pn_chars_base = _{
            ascii_alpha | ['\u{c0}'..'\u{d6}'] | ['\u{d8}'..'\u{f6}'] | ['\u{f8}'..'\u{2ff}']
            | ['\u{370}'..'\u{37d}'] | ['\u{37f}'..'\u{7ff}'] | ['\u{800}'..'\u{1fff}']
            | ['\u{200c}'..'\u{200d}'] | ['\u{2c00}'..'\u{2fef}'] | ['\u{3001}'..'\u{d7ff}']
            | ['\u{f900}'..'\u{fdcf}'] | ['\u{fdf0}'..'\u{fffd}'] | ['\u{10000}'..'\u{effff}']
        }
        pn_chars_u =    _{ pn_chars_base | ["_"] | [":"] }
        pn_chars =      _{ pn_chars_u | ["-"] | ascii_digits | ["\u{B7}"] | ['\u{300}'..'\u{36F}'] | ['\u{203F}'..'\u{2040}'] }

        hex =           _{ ['0'..'9'] | ['A'..'F'] | ['a'..'f'] }
        ascii_alpha =   _{ ['A'..'Z'] | ['a'..'z'] }
        ascii_digits =  _{ ['0'..'9'] }

        comment =    _{ ["#"] ~ (!(eol | eoi) ~ any)* }
        whitespace = _{ (["\u{9}"] | ["\u{20}"])+ } // whitespace gets run between all rules
    }

    process! {
        quads(&self) -> Vec<Quad> {
            (mut list: _expression()) => {
                let mut vec = Vec::new();

                while let Some(quad) = list.pop_front() {
                    vec.push(quad);
                }

                vec
            }
        }

        _expression(&self) -> LinkedList<Quad> {
            (_: statement, mut head: _statement(), mut tail: _expression()) => {
                let quad = Quad {
                    subject:     head.pop_front().unwrap(),
                    predicate:   head.pop_front().unwrap(),
                    object:      head.pop_front().unwrap(),
                    graph_label: head.pop_front().unwrap()
                };
                tail.push_front(quad);
                tail
            },
            () => {
                LinkedList::new()
            }
        }

        _statement(&self) -> LinkedList<Node> {
            (_: subject, head: _node(), mut tail: _statement()) => {
                tail.push_front(head);
                tail
            },
            (_: predicate, head: _node(), mut tail: _statement()) => {
                tail.push_front(head);
                tail
            },
            (_: object, head: _node(), mut tail: _statement()) => {
                tail.push_front(head);
                tail
            },
            (_: graph_label, head: _node(), mut tail: _statement()) => {
                tail.push_front(head);
                tail
            },
            () => {
                LinkedList::new()
            }
        }

        _node(&self) -> Node {
            (&input: blank_node_label) => {
                Node::BlankNodeLabel { label: input.to_string() }
            },
            (&input: iriref) => {
                Node::IriRef { iri: input.to_string() }
            }
        }
    }
}

pub fn parse(input: &str) -> Vec<Quad> {
    let mut parser = Rdp::new(StringInput::new(input));
    parser.expression();
    parser.quads()
}
