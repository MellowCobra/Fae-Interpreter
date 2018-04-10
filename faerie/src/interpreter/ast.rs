use interpreter::token::Token;

pub trait AST {

}

pub struct BinOp {
    left: Num,
    token: Token,
    op: Token,
    right: Num
}

impl BinOp {
    pub fn new(left: Num, op: &Token, right: Num) -> BinOp {
        BinOp {
            left, 
            op: op.clone(),
            token: op.clone(),
            right
        }
    }
}

pub struct Num {
    token: Token,
    value: i32
}

impl Num {
    pub fn new(token: &Token) -> Num {
        Num {
            token: token.clone(),
            value: token.value
        }
    }
}