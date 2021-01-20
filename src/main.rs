use nasl_transpiler::lexer::Lexer;
use nasl_transpiler::token::Token;

use std::fs;
use std::io;

fn main() {
    let data = fs::read_to_string("asd.nasl").unwrap();
    let mut lexer = Lexer::new(data.as_str());
    loop {
        let data = lexer.next_token();
        if data.is_err() {
            println!("{}", data.unwrap_err());
            break;
        }
        let data = data.unwrap();
        if data == Token::Eof {
            break;
        }
        println!("{:?}", data);
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);
    }
}
