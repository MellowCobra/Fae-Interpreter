# CFG for Fae Programming Language

## Terminals
* LPR `(`
* RPR `)`
* MUL `*`
* DIV `/`
* ADD `+`
* SUB `-`
* INTEGER `[0..9]*`

expr -> term ((MUL|DIV) term)*

term -> factor ((ADD|SUB) factor)*

factor -> (ADD|SUB) factor | INTEGER | LPR expr RPR