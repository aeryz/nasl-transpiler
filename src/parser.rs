use super::lexer::Lexer;
use super::ast::*;
use super::token::*;
use std::collections::HashMap;

enum Precedence {
    Postfix,        // () [] -> . var++ var--
    Prefix,         // +var -var ! ~ ++var --var
    Multiplicative, // * -
    Additive,       // + -
    Shift,          // >> <<
    Relational,     // < <= > >= >< >!< =~ !~
    Equality,       // == !=
    BitwiseAnd,     //  &
    BitwiseXor,     // ^
    BitwiseOr,      // |
    LogicalAnd,     // &&
    LogicalOr,      // ||
    Assignment,     // = += -= *= %= >>= <<= ^=
    Comma           // ,
}

type InfixFn<'a> = fn(&mut Parser<'a>, lhs: Expression<'a>) -> Result<Statement, String>;
type PrefixFn<'a> = fn(&mut Parser<'a>) -> Result<Statement, String>;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    infix_fns: HashMap<TokenType, InfixFn<'a>>,
    prefix_fns: HashMap<TokenType, PrefixFn<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(data: &'a str) -> Self {
        let mut infix_fns = HashMap::new();
        let mut prefix_fns = HashMap::new();
        prefix_fns.insert(TokenType::Ident, Parser::parse_identifier);
        Self {
            lexer: Lexer::new(data),
            infix_fns,
            prefix_fns,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>, String> {
        let mut stmts = Vec::new();
        while let Some(stmt) = self.parse_statement() {
            stmts.push(stmt?);
        }
        Ok(stmts)
    }

    fn parse_statement(&mut self) -> Option<Result<Statement, String>> {
        match self.lexer.next_token() {
            _ => None
        }
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {
        unimplemented!()
    }

    fn parse_identifier(&mut self) -> Result<Expression, String> {
        unimplemented!()
    }

    fn expect_token(&mut self, token: Token) -> Result<(), ()> {
        Ok(())
    }

    fn next_token(&mut self) -> Result<Token, String> {
        self.lexer.next_token()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser() {
        let parser = Parser::new("a + b * c;");
        let stmts = parser.parse().unwrap();
        println!("{:?}", stmts);
    }

    #[test]
    fn if_statement() {
        let parser = Parser::new("if (a + 23 * b + c) {} else {}");
        let stmts = parser.parse().unwrap();
        assert_eq!(stmts, vec![
            Statement::If {
                cond: Box::new(
                    Expression::Infix {
                        l_expr: Box::new(Expression::Identifier("a")),
                        op: "+",
                        r_expr: Box::new(Expression::Infix {
                            l_expr: Box::new(Expression::Number(23)),
                            op: "*",
                            r_expr: Box::new(Expression::Identifier("c")),
                        })
                    }
                ),
                if_block: Box::new(Statement::Block {
                    statements: Vec::new(),
                    }
                ),
                else_block: Some(Box::new(Statement::Block {
                    statements: Vec::new(),
                }))
            }]);
    }
}
