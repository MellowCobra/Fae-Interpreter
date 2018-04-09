use interpreter::data_type::Type;
use interpreter::token::Token;
use interpreter::errors::unknown_token_error;

const NULLCHAR: char = '\0';

#[derive(Debug)]
pub struct Lexer {
    text: Vec<char>,
    pub pos: usize,
    line: i32,
    current_char: char
}

impl Lexer {
    pub fn new(text: Vec<char>) -> Lexer {
        assert!(text.len() != 0);
        let current_char = text[0];
        Lexer {
            text,
            pos: 0,
            line: 0,
            current_char
        }
    }
}

impl Lexer {
    pub fn get_next_token(&mut self) -> Token {
        while self.current_char != NULLCHAR {
            if self.current_char.is_whitespace() {
                self.skip_whitespace()
            } else if self.current_char.is_digit(10) {
                let value = self.integer();
                return Token::new(Type::INTEGER, value)
            } else if self.current_char == '(' {
                self.advance();
                return Token::new(Type::LPR, 0)
            } else if self.current_char == ')' {
                self.advance();
                return Token::new(Type::RPR, 0)    
            } else if self.current_char == '+' {
                self.advance();
                return Token::new(Type::ADD, 0)
            } else if self.current_char == '-' {
                self.advance();
                return Token::new(Type::SUB, 0)
            } else if self.current_char == '*' {
                self.advance();
                return Token::new(Type::MUL, 0) 
            } else if self.current_char == '/' {
                self.advance();
                return Token::new(Type::DIV, 0)
            } else {
                unknown_token_error::throw(0, self.pos, self.current_char);
            }
        }

        Token::new(Type::EOF, 0)
    }

    fn advance(&mut self) {
        self.pos += 1;

        if self.pos >= self.text.len() {
            self.current_char = NULLCHAR;
        } else {
            self.current_char = self.text[self.pos];
        }
    }

    fn skip_whitespace(&mut self) {
        while self.current_char != NULLCHAR && self.current_char.is_whitespace() {
            self.advance();
        }
    }

    fn integer(&mut self) -> i32 {
        let mut value = String::new();
        while self.current_char != NULLCHAR && self.current_char.is_digit(10) {
            value.push(self.current_char);
            self.advance();
        }

        return value.parse::<i32>().unwrap();
    }
}