use std::fs::File;
use std::io::prelude::*;
extern crate faerie;

// use faerie::data_type::Type;
// use faerie::token::Token;
// use faerie::interpreter::Parser;
use faerie::interpreter::Interpreter;

fn main() {
    let mut file = File::open("./program.fae").expect("file not found");
    let mut program = String::new();

    file.read_to_string(&mut program).expect("failed to read from file");
    let lines: Vec<&str> = program.split("\n").collect();

    for line in lines {
        // let mut parser = Parser::new(line);

        // let result = parser.parse();
        // result.print();

        let mut interpreter = Interpreter::new(line);
        let result = interpreter.interpret();

        println!("{} is {:#?}", line, result)
    }
}
