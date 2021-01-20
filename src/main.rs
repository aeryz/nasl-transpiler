use nasl_transpiler::lexer::Lexer;
use nasl_transpiler::token::Token;

use std::fs;
use std::io;

fn main() {
    let data = fs::read_to_string("asd.nasl").unwrap();
    let mut lexer = Lexer::new(data.as_str());
    while let Ok(data) = lexer.next_token() {
        println!("{:?}", data);
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);
    }
}
