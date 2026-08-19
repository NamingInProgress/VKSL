use crate::token::tokenizer::Tokenizer;
use mvutils::utils::UnwrapDisplay;
use parser::Parser;
use std::path::Path;
use crate::scope::ScopeResolver;

pub mod ast;
pub mod parser;
pub mod token;
pub mod scope;

fn main() {
    // for token in Tokenizer::from_str(include_str!("../simple.vksl").to_string()) {
    //     println!("{token:?}")
    // }

    let source = include_str!("../simple.vksl").chars();
    let tokens = Tokenizer::new(source, Some(Path::new("simple.vksl").into()));
    let parser = Parser::new(tokens);
    let result = parser.parse().unwrapd();
    let scope_thingy = ScopeResolver::new(result);
    let result = scope_thingy.parse();
    for stmt in result.unwrapd() {
        println!("{stmt:#?}");
    }
}