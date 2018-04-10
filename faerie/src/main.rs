use std::fs::File;
use std::io::prelude::*;
extern crate faerie;

// use faerie::data_type::Type;
// use faerie::token::Token;
use faerie::interpreter::Parser;

fn main() {
    let mut file = File::open("./program.fae").expect("file not found");
    let mut program = String::new();

    file.read_to_string(&mut program).expect("failed to read from file");
    let lines: Vec<&str> = program.split("\n").collect();

    for line in lines {
        let mut parser = Parser::new(line);

        let result = parser.parse();
        // result.print();
        println!("{} is {:#?}", line, result)
    }
}
