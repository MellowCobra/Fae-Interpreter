#[derive(Debug,PartialEq,Clone)]
pub enum Type {
    INTEGER, // [1..9]*
    LPR, // (
    RPR, // )
    ADD, // +
    SUB, // -
    MUL, // *
    DIV, // /
    EOF, // \0
    EMPTY // usually an error
}