use interpreter::token::Token;
use interpreter::data_type::Type;

pub trait Visit {
    fn visit(&self) -> i32;
}


pub struct BinOp {
    token: Token,
    left: Box<Visit>,
    op: Token,
    right: Box<Visit>
}

impl BinOp {
    pub fn new(token: Token, left: Box<Visit>, right: Box<Visit>) -> BinOp {
        BinOp {
            token: token.clone(),
            op: token,
            left,
            right
        }
    } 
}

impl Visit for BinOp {
    fn visit(&self) -> i32 {
        match self.op._type {
            Type::ADD => self.left.visit() + self.right.visit(),
            Type::SUB => self.left.visit() - self.right.visit(),
            Type::MUL => self.left.visit() * self.right.visit(),
            Type::DIV => self.left.visit() / self.right.visit(),
            _ => panic!("Bad op token {:?} in binary operation", self.op)
        }
    }
}

pub struct Num {
    token: Token,
    value: i32
}

impl Num {
    pub fn new(token: Token) -> Num {
        Num {
            value: token.value,
            token: token.clone()
        }
    }
}

impl Visit for Num {
    fn visit(&self) -> i32 {
        self.value
    }
}

pub struct UnaryOp {
    token: Token,
    op: Token,
    expr: Box<Visit>
}

impl UnaryOp {
    pub fn new(token: Token, expr: Box<Visit>) -> UnaryOp {
        UnaryOp {
            token: token.clone(),
            op: token,
            expr
        }
    }
}

impl Visit for UnaryOp {
    fn visit(&self) -> i32 {
        match self.op._type {
            Type::ADD => self.expr.visit(),
            Type::SUB => - self.expr.visit(),
            _ => panic!("Bad op token {:?} in unary operation", self.op)
        }
    }
}