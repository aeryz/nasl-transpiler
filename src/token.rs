#[derive(Debug, PartialEq, Eq, Hash)]
pub enum TokenType {
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
    Ident,
    PureStr,
    ImpureStr,
    Colon,

    Num,

    Eof,
    Comment,

    Function,
    Null,
    True,
    False,
    Return,
    For,
    While,
    Break,
    Continue,
    Foreach,
    Include,
    LocalVar,
    GlobalVar,
    Repeat,
    Until,
}

pub struct Token<'a> {
    pub literal: &'a str,
    pub ty: TokenType,
}

impl<'a> Token<'a> {
    pub fn from_ident(literal: &'a str) -> Self {
        let ty = match literal {
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "function" => TokenType::Function,
            "NULL" => TokenType::Null,
            "TRUE" => TokenType::True,
            "FALSE" => TokenType::False,
            "return" => TokenType::Return,
            "for" => TokenType::For,
            "while" => TokenType::While,
            "break" => TokenType::Break,
            "continue" => TokenType::Continue,
            "foreach" => TokenType::Foreach,
            "include" => TokenType::Include,
            "local_var" => TokenType::LocalVar,
            "global_var" => TokenType::GlobalVar,
            "repeat" => TokenType::Repeat,
            "until" => TokenType::Until,
            _ => TokenType::Ident,
        };
        TokenType { literal, ty }
    }

    // TODO: This only supports operators
    pub fn from_str(literal: &'a str) -> Self {
        let ty = match literal {
            "+" => TokenType::Plus,
            "-" => TokenType::Minus,
            "*" => TokenType::Mul,
            "/" => TokenType::Div,
            "%" => TokenType::Mod,
            "=" => TokenType::Assign,
            "==" => TokenType::Equ,
            "!=" => TokenType::Nequ,
            "&&" => TokenType::And,
            "||" => TokenType::Or,
            ">" => TokenType::Gt,
            "<" => TokenType::Lt,
            ">=" => TokenType::Gte,
            "<=" => TokenType::Lte,
            "!" => TokenType::Not,
            "&" => TokenType::BwAnd,
            "|" => TokenType::BwOr,
            "^" => TokenType::BwXor,
            "+=" => TokenType::PlusEq,
            "-=" => TokenType::MinusEq,
            "/=" => TokenType::DivEq,
            "%=" => TokenType::ModEq,
            "*=" => TokenType::MulEq,
            "^=" => TokenType::XorEq,
            ">>" => TokenType::Shr,
            "<<" => TokenType::Shl,
            ">>=" => TokenType::ShrEq,
            "<<=" => TokenType::ShlEq,
            ">>>" => TokenType::Ushr,
            "++" => TokenType::Incr,
            "--" => TokenType::Decr,
            "**" => TokenType::Pow,
            "><" => TokenType::Substr,
            ">!<" => TokenType::NSUBSTR,
            "=~" => TokenType::ReMatch,
            "!~" => TokenType::NreMatch,
            _ => panic!("Unknown token {}", data),
        };
        TokenType { literal, ty }
    }
}
