#[derive(Debug,PartialEq,Clone)]
pub enum Type {
    INTEGER, // 1,2,23,185, etc
    LPR, // (
    RPR, // )
    ADD, // +
    SUB, // -
    MUL, // *
    DIV, // /
    EOF // \0
}