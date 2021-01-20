#[derive(Debug, PartialEq, Eq)]
pub enum Token<'a> {
    /* Operators */
    Plus,     // +
    Minus,    // -
    Mul,      // *
    Div,      // /
    Mod,      // %
    Assign,   // =
    Equ,      // ==
    Nequ,     // !=
    And,      // &&
    Or,       // ||
    Gt,       // >
    Lt,       // <
    Gte,      // >=
    Lte,      // <=
    Not,      // !
    BwAnd,    // &
    BwOr,     // |
    BwXor,    // ^
    PlusEq,   // +=
    MinusEq,  // -=
    DivEq,    // /=
    ModEq,    // %=
    MulEq,    // *=
    XorEq,    // ^=
    Shr,      // >>
    Shl,      // <<
    ShrEq,    // >>=
    ShlEq,    // <<=
    Ushr,     // >>>
    Incr,     // ++
    Decr,     // --
    Pow,      // **
    Substr,   // ><
    NSUBSTR,  // >!<
    ReMatch,  // =~
    NreMatch, // !~

    If,
    Else,
    Lparan,
    Rparan,
    Lbrace,
    Rbrace,

    Lbracket,
    Rbracket,

    SemiColon,
    Comma,
    Ident(&'a str),
    PureStr(&'a str),
    ImpureStr(&'a str),
    Colon,

    Num(i32),

    Eof,
    Comment,
}

impl<'a> Token<'a> {
    pub fn from_ident(data: &'a str) -> Self {
        match data {
            "if" => Token::If,
            "else" => Token::Else,
            _ => Token::Ident(data),
        }
    }

    // TODO: This only supports operators
    pub fn from_str(data: &'a str) -> Self {
        match data {
            "+" => Token::Plus,
            "-" => Token::Minus,
            "*" => Token::Mul,
            "/" => Token::Div,
            "%" => Token::Mod,
            "=" => Token::Assign,
            "==" => Token::Equ,
            "!=" => Token::Nequ,
            "&&" => Token::And,
            "||" => Token::Or,
            ">" => Token::Gt,
            "<" => Token::Lt,
            ">=" => Token::Gte,
            "<=" => Token::Lte,
            "!" => Token::Not,
            "&" => Token::BwAnd,
            "|" => Token::BwOr,
            "^" => Token::BwXor,
            "+=" => Token::PlusEq,
            "-=" => Token::MinusEq,
            "/=" => Token::DivEq,
            "%=" => Token::ModEq,
            "*=" => Token::MulEq,
            "^=" => Token::XorEq,
            ">>" => Token::Shr,
            "<<" => Token::Shl,
            ">>=" => Token::ShrEq,
            "<<=" => Token::ShlEq,
            ">>>" => Token::Ushr,
            "++" => Token::Incr,
            "--" => Token::Decr,
            "**" => Token::Pow,
            "><" => Token::Substr,
            ">!<" => Token::NSUBSTR,
            "=~" => Token::ReMatch,
            "!~" => Token::NreMatch,
            _ => panic!("Unknown token {}", data),
        }
    }
}
