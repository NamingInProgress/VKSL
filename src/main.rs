use crate::parser::expr::Parser;
use crate::token::tokenizer::Tokenizer;

pub mod token;
pub mod ast;
pub mod parser;

fn main() {
    // for token in Tokenizer::from_str(include_str!("../simple.vksl").to_string()) {
    //     println!("{token:?}")
    // }

    let tokens = Tokenizer::from_str(include_str!("../simple.vksl").to_string());
    let parser = Parser::new(tokens);
    let result = parser.parse();
    for stmt in result.unwrap() {
        println!("{stmt:#?}");
    }
}