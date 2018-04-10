use interpreter::token::Token;

#[derive(Debug)]
pub enum AST {
    BinOp { left: Box<AST>, token: Token, op: Token, right: Box<AST> },
    Num { token: Token, value: i32 }
}