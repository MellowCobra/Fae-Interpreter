# Fae-Interpreter
Interpreter for the Fae programming language

> This interpreter is being built as an exercise to better learn and understand programming language design and construction.

The extent of the Fae programming language thus far is limited to a string of addition, subtraction, multiplication, and division operations for unsigned integers. The interpreter does utilize order of operations, and allow parentheses for grouping of operations.

Currently this interpreter consists of:
* A lexer that reads a single string and converts to a token stream
* A parser that reads the token stream and converts into an Abstract Syntax Tree
* An interpreter which traverses the AST and produces the result of the arithmetic expression

This interpreter was made so far following [Ruslan's Let's Build a Simple Interpreter](https://ruslanspivak.com/lsbasi-part1/)