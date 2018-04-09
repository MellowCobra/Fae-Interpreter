mod data_type;
mod token;
mod lexer;
mod errors;

use self::data_type::Type;
use self::token::Token;
use self::lexer::Lexer;
use self::errors::parsing_error;
use self::errors::unmatched_token_error;

#[derive(Debug)]
pub struct Interpreter {
    current_token: Token,
    lexer: Lexer
}

impl Interpreter {
    pub fn new(program: &str) -> Interpreter {
        let text: Vec<char> = String::from(program).chars().collect();
        let mut lexer = Lexer::new(text);
        let current_token = lexer.get_next_token();
        Interpreter {
            lexer,
            current_token
        }
    }
}

impl Interpreter {

    fn eat(&mut self, token_type: Type) {
        if self.current_token._type == token_type {
            self.current_token = self.lexer.get_next_token();
        } else {
            unmatched_token_error::throw(0, self.lexer.pos, &token_type, &self.current_token._type);
        }
    }

    pub fn expr(&mut self) -> i32 {
        let mut result = self.term();

        // println!("first term of expr is {} and current_char is {}", result, self.current_char);

        while self.current_token._type == Type::ADD || self.current_token._type == Type::SUB {
         
            let op = self.op();
            let right = self.term();

            result = match op {
                Type::ADD => result + right,
                Type::SUB => result - right,
                _ => {
                    unmatched_token_error::throw(0, self.lexer.pos, &Type::ADD, &op);
                    0
                }
            }
        }

        result
    }

    fn term(&mut self) -> i32 {

        let mut result = self.factor();

        // println!("first factor of term is {} and current_char is {}", result, self.current_char);

        while self.current_token._type == Type::MUL || self.current_token._type == Type::DIV {
            
            let op = self.op();
            let right = self.factor();

            result = match op {
                Type::MUL => result * right,
                Type::DIV => result / right,
                _ => {
                    unmatched_token_error::throw(0, self.lexer.pos, &Type::MUL, &op);
                    0
                }
            }
        }

        result
    }

    fn factor(&mut self) -> i32 {
        if self.current_token._type == Type::INTEGER {
            let t = self.current_token.clone();
            self.eat(Type::INTEGER);
            t.value
        } else if self.current_token._type == Type::LPR {
            self.eat(Type::LPR);
            let result = self.expr();
            self.eat(Type::RPR);
            result
        } else {
            unmatched_token_error::throw(0, self.lexer.pos, &Type::INTEGER, &self.current_token._type);
            0
        }
    }

    fn op(&mut self) -> Type {
        let op = self.current_token._type.clone();
        
        match op {
            Type::ADD => self.eat(Type::ADD),
            Type::SUB => self.eat(Type::SUB),
            Type::MUL => self.eat(Type::MUL),
            Type::DIV => self.eat(Type::DIV),
            _ => parsing_error::throw(0, self.lexer.pos)
        }

        op
    }
}