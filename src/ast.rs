use std::collections::HashMap;

pub enum Expression<'a> {
    Identifier(&'a str),
    Number(i32),
    Prefix {
        op: &'a str,
        expr: Box<Expression<'a>>
    },
    Infix {
        l_expr: Box<Expression<'a>>,
        op: &'a str,
        r_expr: Box<Expression<'a>>
    },
    Postfix {
        expr: &'a str,
        op: Box<Expression<'a>>
    },
    // $ident($anon_args, $named_args);
    FunctionCall {
        ident: &'a str,
        named_args: HashMap<&'a str, Expression<'a>>,
        anon_args: Vec<Expression<'a>>
    },
    // $ident = $expr;
    Assign {
        ident: &'a str,
        expr: Box<Expression<'a>>
    }
}

pub enum Statement<'a> {
    // if ($cond) $if_block
    // else $else_block
    If {
        cond: Box<Expression<'a>>,
        if_block: Box<Statement<'a>>,
        else_block: Option<Box<Statement<'a>>>
    },
    // while ($cond) $block;
    While {
        cond: Box<Expression<'a>>,
        block: Box<Statement<'a>>
    },
    // for ($init_expr; $cond; $end_expr) $block;
    For {
        init_expr: Option<Box<Expression<'a>>>,
        cond: Option<Box<Expression<'a>>>,
        end_expr: Option<Box<Expression<'a>>>,
        block: Box<Statement<'a>>
    },
    // foreach $var ($array) $block;
    Foreach {
        var: Box<Statement<'a>>,
        array: Box<Statement<'a>>,
        block: Box<Statement<'a>>
    },
    // repeat $block; until ($cond);
    Repeat {
        block: Box<Statement<'a>>,
        cond: Box<Expression<'a>>,
    },
    // function $ident ($args) { body }
    FunctionDef {
        ident: &'a str,
        args: Vec<Expression<'a>>,
        body: Box<Statement<'a>>,
    },
    // return $expr;
    Return {
        expr: Option<Box<Expression<'a>>>,
    },
    Block {
        statements: Vec<Statement<'a>>,
    },

}