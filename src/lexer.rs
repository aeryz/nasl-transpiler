//! Lexer (a.k.a. Tokenizer) of the Nasl language.
//!
//! For now, it supports unicode but it might be changed
//! in the future for performance reasons. We already know
//! that the original interpreter uses Latin encoding so
//! we don't need to pay for unicode.

use crate::token::Token;
use std::str::Chars;

type LResult<T> = Result<T, String>;

#[derive(Debug)]
pub struct Lexer<'a> {
    data: &'a str,

    // Using an iterator here because immediate access does not exist
    // in the unicode.
    peek_cursor: Chars<'a>,

    cur_pos: usize,
    cur_char: Option<char>,
    peek_char: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(data: &'a str) -> Self {
        let mut peek_cursor = data.chars();
        peek_cursor.next();
        peek_cursor.next();
        Lexer {
            data,
            peek_cursor,
            cur_pos: 0,
            cur_char: data.chars().next(),
            peek_char: data.chars().skip(1).next(),
        }
    }

    pub fn next_token(&mut self) -> LResult<Token<'a>> {
        self.eat_whitespace_or_comment();
        let token = match self.cur_char {
            Some('(') => Token::Lparan,
            Some(')') => Token::Rparan,
            Some('{') => Token::Lbrace,
            Some('}') => Token::Rbrace,
            Some('[') => Token::Lbracket,
            Some(']') => Token::Rbracket,
            Some(';') => Token::SemiColon,
            Some(':') => Token::Colon,
            Some('"') => Token::ImpureStr(self.read_impure_str()?),
            Some('\'') => Token::PureStr(self.read_pure_str()?),
            Some(',') => Token::Comma,
            Some('+') => {
                let token = match self.peek_char {
                    Some('+') => Token::Incr,
                    Some('=') => Token::PlusEq,
                    _ => Token::Plus,
                };
                if token != Token::Plus {
                    let _ = self.read_char();
                }
                token
            }
            Some('-') => {
                let token = match self.peek_char {
                    Some('-') => Token::Decr,
                    Some('=') => Token::MinusEq,
                    _ => Token::Minus,
                };
                if token != Token::Minus {
                    let _ = self.read_char();
                }
                token
            }
            Some('*') => {
                let token = match self.peek_char {
                    Some('*') => Token::Pow,
                    Some('=') => Token::MulEq,
                    _ => Token::Mul,
                };
                if token != Token::Mul {
                    let _ = self.read_char();
                }
                token
            }
            Some('/') => {
                let token = match self.peek_char {
                    Some('=') => Token::DivEq,
                    _ => Token::Div,
                };
                if token != Token::Div {
                    let _ = self.read_char();
                }
                token
            }
            Some('%') => {
                let token = match self.peek_char {
                    Some('=') => Token::ModEq,
                    _ => Token::Mod,
                };
                if token != Token::Mod {
                    let _ = self.read_char();
                }
                token
            }
            Some('=') => {
                let token = match self.peek_char {
                    Some('=') => Token::Equ,
                    Some('~') => Token::ReMatch,
                    _ => Token::Assign,
                };
                if token != Token::Assign {
                    let _ = self.read_char();
                }
                token
            }
            Some('!') => {
                let token = match self.peek_char {
                    Some('=') => Token::Nequ,
                    Some('~') => Token::NreMatch,
                    _ => Token::Not,
                };
                if token != Token::Not {
                    let _ = self.read_char();
                }
                token
            }
            Some('&') => {
                if self.peek_char == Some('&') {
                    let _ = self.read_char();
                    Token::And
                } else {
                    Token::BwAnd
                }
            }
            Some('|') => {
                if self.peek_char == Some('|') {
                    let _ = self.read_char();
                    Token::Or
                } else {
                    Token::BwOr
                }
            }
            Some('^') => {
                if self.peek_char == Some('=') {
                    let _ = self.read_char();
                    Token::XorEq
                } else {
                    Token::BwXor
                }
            }
            Some('>') => {
                let token = match self.peek_char {
                    Some('<') => {
                        let _ = self.read_char();
                        Token::Substr
                    }
                    Some('=') => {
                        let _ = self.read_char();
                        Token::Gte
                    }
                    Some('>') => {
                        let _ = self.read_char();
                        let tok = match self.peek_char {
                            Some('=') => Token::ShrEq,
                            Some('>') => Token::Ushr,
                            _ => Token::Shr,
                        };
                        if tok != Token::Shr {
                            let _ = self.read_char();
                        }
                        tok
                    }
                    Some('!') => {
                        let _ = self.read_char();
                        let tok = if self.peek_char == Some('<') {
                            let _ = self.read_char();
                            Token::NSUBSTR
                        } else {
                            return Err(format!("Expected '<', got {:?}", self.peek_char));
                        };
                        let _ = self.read_char();
                        tok
                    }
                    _ => Token::Gt,
                };
                token
            }
            Some('<') => {
                let token = match self.peek_char {
                    Some('=') => Token::Lte,
                    Some('<') => {
                        let _ = self.read_char();
                        match self.peek_char {
                            Some('=') => {
                                let _ = self.read_char();
                                Token::ShlEq
                            }
                            _ => Token::Shl,
                        }
                    }
                    _ => Token::Lt,
                };
                if token != Token::Lt {
                    let _ = self.read_char();
                }
                token
            }
            Some(ch) => {
                if ch.is_ascii_alphabetic() || ch == '_' {
                    let key = self.read_identifier();
                    return Ok(Token::from_ident(key));
                } else if ch.is_numeric() {
                    let key = self.read_number()?;
                    return Ok(Token::Num(key));
                }
                return Err(format!("Unexpected character {}", ch));
            }
            None => Token::Eof,
        };
        let _ = self.read_char();
        Ok(token)
    }
}

impl<'a> Lexer<'a> {
    /// Read `pure` and `impure` strings
    ///
    /// `Impure` strings are entered between double qoutes and are not converted.
    /// `Pure` strings are returned between single quotes and escapte sequences are transformed.
    ///
    fn read_impure_str(&mut self) -> LResult<&'a str> {
        let _ = self.read_char();
        let cur_pos = self.cur_pos;
        while let Some(ch) = self.cur_char {
            if ch == '"' {
                return Ok(&self.data[cur_pos..self.cur_pos]);
            }
            let _ = self.read_char();
        }
        Err(String::from("Unexpected eof while reading a string"))
    }

    fn read_pure_str(&mut self) -> LResult<&'a str> {
        let _ = self.read_char();
        let cur_pos = self.cur_pos;
        let mut on_escape = false;
        while let Some(ch) = self.cur_char {
            if ch == '\\' && !on_escape {
                on_escape = true;
            } else if on_escape {
                match ch {
                    'n' | 't' | 'v' | 'r' | '\'' | '"' | 'b' | '\\' => {}
                    _ => return Err(format!("Unexpected escape character. \
                                             Expected one of '\\n, \\t, \\v, \\r, \\', \\\", \
                                             \\b', got \\{}", self.cur_char.unwrap()))
                }
                on_escape = false;
            } else if ch == '\'' {
                return Ok(&self.data[cur_pos..self.cur_pos]);
            }

            let _ = self.read_char();
        }
        Err(String::from("Unexpected eof while reading a string"))
    }

    fn eat_whitespace_or_comment(&mut self) {
        let mut in_comment = false;
        while let Some(ch) = self.cur_char {
            if in_comment {
                if ch == '\n' {
                    in_comment = false;
                }
                let _ = self.read_char();
                continue;
            }
            if ch == '#' {
                in_comment = true;
                continue;
            }
            if ch != '\t' && ch != '\r' && ch != '\x0C' && ch != ' ' && ch != '\n' {
                break;
            }
            let _ = self.read_char();
        }
    }

    fn read_char(&mut self) -> Option<char> {
        self.cur_char = self.peek_char;
        self.peek_char = self.peek_cursor.next();
        self.cur_pos += 1;
        self.cur_char
    }

    fn read_identifier(&mut self) -> &'a str {
        let cur_pos = self.cur_pos;
        while let Some(ch) = self.cur_char {
            if !ch.is_ascii_alphanumeric() && ch != '_' {
                break;
            }
            let _ = self.read_char();
        }
        &self.data[cur_pos..self.cur_pos]
    }

    fn read_number(&mut self) -> LResult<i32> {
        let cur_pos = self.cur_pos;
        while let Some(ch) = self.cur_char {
            if !ch.is_numeric() {
                break;
            }
            let _ = self.read_char();
        }
        self.data[cur_pos..self.cur_pos]
            .parse::<i32>()
            .map_err(|_| {
                format!(
                    "{} is not a valid number.",
                    &self.data[cur_pos..self.cur_pos]
                )
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operators_pass() {
        let ops = [
            "+", "-", "*", "/", "%", "=", "==", "!=", "&&", "||", ">", "<", ">=", "<=", "!", "&",
            "|", "^", "+=", "-=", "/=", "%=", "*=", "^=", ">>", "<<", ">>=", "<<=", ">>>", "++",
            "--", "**", "><", ">!<", "=~", "!~",
        ];
        let data = ops.join(" ");
        let mut lexer = Lexer::new(data.as_str());
        let mut ops_iter = ops.iter();
        while let Ok(tok) = lexer.next_token() {
            if tok == Token::Eof {
                break;
            }
            assert_eq!(Token::from_str(ops_iter.next().unwrap()), tok);
        }
        assert_eq!(ops_iter.next(), None);
    }

    #[test]
    fn escape_chars() {
        let data = r#"
            '\n' '\r' '\t' '\b' '\\' '\'' '\"' '\v' '\F'
        "#;

        let expected = [
            "\\n", "\\r", "\\t", "\\b", "\\\\", "\\'", "\\\"", "\\v",
        ];

        let mut lexer = Lexer::new(data);
        for exp in &expected {
            assert_eq!(Ok(Token::PureStr(exp)), lexer.next_token());
        }
        assert!(lexer.next_token().is_err());
    }
}
