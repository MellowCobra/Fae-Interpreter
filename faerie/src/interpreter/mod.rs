mod ast;
mod data_type;
mod errors;
mod lexer;
mod parser;
mod token;

use self::ast::BinOp;
use self::parser::Parser;

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
        let ast = self.parser.parse();
        ast.visit()
    }
}