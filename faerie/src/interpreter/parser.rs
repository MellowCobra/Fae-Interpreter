use interpreter::ast::Visit;
use interpreter::ast::BinOp;
use interpreter::ast::Num;
use interpreter::data_type::Type;
use interpreter::token::Token;
use interpreter::lexer::Lexer;
use interpreter::errors::unmatched_token_error;

#[derive(Debug)]
pub struct Parser {
    current_token: Token,
    lexer: Lexer
}

impl Parser {
    pub fn new(program: &str) -> Parser {
        let text: Vec<char> = String::from(program).chars().collect();
        let mut lexer = Lexer::new(text);
        let current_token = lexer.get_next_token();
        Parser {
            lexer,
            current_token
        }
    }
}

impl Parser {

    pub fn parse(&mut self) -> Box<Visit> {
        self.expr()
    }

    fn eat(&mut self, token_type: Type) {
        if self.current_token._type == token_type {
            self.current_token = self.lexer.get_next_token();
        } else {
            unmatched_token_error::throw(0, self.lexer.pos, &token_type, &self.current_token._type);
        }
    }

    fn expr(&mut self) -> Box<Visit> {
        let mut node = self.term();

        while self.current_token._type == Type::ADD || self.current_token._type == Type::SUB {
            let token = self.current_token.clone();

            match token._type {
                Type::ADD => self.eat(Type::ADD),
                Type::SUB => self.eat(Type::SUB),
                _ => {
                    unmatched_token_error::throw(0, self.lexer.pos, &Type::ADD, &token._type)
                }
            }
            
            let left = node;
            node = Box::new(BinOp::new(token, left, self.term()));
            // node = AST::BinOp { 
            //     left: Box::new(node),
            //     op: token.clone(), 
            //     token,
            //     right: Box::new(self.term())
            // }
        }

        node
    }

    fn term(&mut self) -> Box<Visit> {
        let mut node = self.factor();

        while self.current_token._type == Type::MUL || self.current_token._type == Type::DIV {
            let token = self.current_token.clone();
            match token._type {
                Type::MUL => self.eat(Type::MUL),
                Type::DIV => self.eat(Type::DIV),
                _ => {
                    unmatched_token_error::throw(0, self.lexer.pos, &Type::MUL, &token._type);
                }
            }

            node = Box::new(BinOp::new(token, node, self.factor()));
            // node = AST::BinOp {
            //     left: Box::new(node),
            //     op: token.clone(),
            //     token, 
            //     right: Box::new(self.factor())
            // }
        }

        node
    }

    fn factor(&mut self) -> Box<Visit> {
        if self.current_token._type == Type::INTEGER {
            let token = self.current_token.clone();
            self.eat(Type::INTEGER);
            // AST::Num {
            //     value: token.value,
            //     token
            // }
            Box::new(Num::new(token))
        } else if self.current_token._type == Type::LPR {
            self.eat(Type::LPR);
            let node = self.expr();
            self.eat(Type::RPR);
            node
        } else {
            unmatched_token_error::throw(0, self.lexer.pos, &Type::INTEGER, &self.current_token._type);
            // AST::Num {
            //     token: Token::new(Type::EMPTY, 0),
            //     value: 0
            // }
            Box::new(Num::new(Token::new(Type::EMPTY, 0)))
        }
    }
}