use std::fs;

use nightcode::frontend::{lexer, parser};

fn main() {
    let source = fs::read_to_string("./test/parser_test1.nc").unwrap();
    let tokens = lexer::tokenize(&source);

    for tok in &tokens {
        println!("{tok}");
    }
    println!("total tokens count: {}", tokens.len());

    match parser::parse(&tokens) {
        Ok(value) => println!("parsed: {value}"),
        Err(error) => error.throw(),
    }
}
