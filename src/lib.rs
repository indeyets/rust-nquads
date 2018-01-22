extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

pub mod types;
use types::*;

mod errors;
use errors::ParseError;

mod parser;
use parser::*;

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
