mod ast;
mod data_type;
mod errors;
mod lexer;
mod parser;
mod token;

use self::ast::AST;
use self::parser::Parser;
use self::data_type::Type;

trait NodeVisitor {
    fn visit(&mut self, node: AST) -> i32;
}

pub struct Interpreter {
    parser: Parser
}

impl Interpreter {
    pub fn new(program: &str) -> Interpreter {
        Interpreter {
            parser: Parser::new(program)
        }
    }

    pub fn interpret(&mut self) -> i32 {
        // self.visit(self.parser.parse())
        let mut ast = self.parser.parse();
        ast.visit()
    }

    // fn visit_BinOp(&mut self, left, op, right) -> i32 {
    //     match op._type {
    //         Type::ADD => self.visit(left.clone()) + self.visit(right.clone()),
    //         Type::SUB => self.visit(left.clone()) - self.visit(right.clone()),
    //         Type::MUL => self.visit(left.clone()) * self.visit(right.clone()),
    //         Type::DIV => self.visit(left.clone()) / self.visit(right.clone()),
    //     }
    // }

    // fn visit_Num(&mut self, ) -> i32 {
    //     node.value
    // }
}

// impl NodeVisitor for Interpreter {
//     fn visit(&mut self, node: AST) -> i32 {
//         match node {
//             BinOp => self.visit_BinOp(node.left.clone(), node.op.clone(), node.right.clone()),
//             Num => self.visit_Num(node.value)
//         }
//     }
// }

impl AST {
    pub fn visit(&mut self) -> i32 {
        match *self {
            AST::BinOp { ref left, ref token, ref op, ref right } => {
                match op._type {
                    Type::ADD => left.clone().visit() + right.clone().visit(),
                    Type::SUB => left.clone().visit() - right.clone().visit(),
                    Type::MUL => left.clone().visit() * right.clone().visit(),
                    Type::DIV => left.clone().visit() / right.clone().visit(),
                    _ => panic!("AST has a bad operation in it!")
                }
            },
            AST::Num {ref token, ref value} => *value
        }
    }
}