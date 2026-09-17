use crate::token::tokenizer::Tokenizer;
use mvutils::utils::UnwrapDisplay;
use parser::Parser;
use std::path::Path;
use crate::scope::{name_res, struct_res};

pub mod parser;
pub mod token;
pub mod scope;

#[macro_export]
macro_rules! critical_error {
    () => {
        panic!("critical error: {}:{}", file!(), line!())
    };
}

fn main() {
    fn parse() -> parser::Result<()> {
        // for token in Tokenizer::from_str(include_str!("../simple.vksl").to_string()) {
        //     println!("{token:?}")
        // }
    
        let source = include_str!("../simple.vksl").chars();
        let tokens = Tokenizer::new(source, Some(Path::new("simple.vksl").into()));
        let parser = Parser::new(tokens);
        let mut result = parser.parse()?;
        let global_scope = struct_res::handle_ast(&mut result)?;
        let result = name_res::conv_ast(result, global_scope.clone())?;
        for stmt in result {
            println!("{stmt:#?}");
        }
    
        println!("=======");
        let g = global_scope.borrow();
        println!("{:#?}", g.symbols);
    
        Ok(())
    }
    parse().unwrapd();
}