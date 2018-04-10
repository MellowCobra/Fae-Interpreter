use interpreter::token::Token;

pub trait AST {
    fn print(self);
}

pub struct BinOp {
    left: AST,
    token: Token,
    op: Token,
    right: AST
}

impl BinOp {
    pub fn new(left: AST, op: &Token, right: AST) -> BinOp {
        BinOp {
            left, 
            op: op.clone(),
            token: op.clone(),
            right
        }
    }
}

impl AST for BinOp {
    fn print(self) {
        println!("left: {}, op: {}, right: {}, token: {}", self.left, self.op, self.right, self.token);
    }
}

pub struct Num {
    token: Token,
    value: i32
}

impl Num {
    pub fn new(token: &Token) -> AST {
        Num {
            token: token.clone(),
            value: token.value
        }
    }
}

impl AST for Num {
    fn print(self) {
        println!("token: {}, value: {}", self.token, self.value);
    }
}