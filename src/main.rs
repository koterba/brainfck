mod compiler;
mod interpreter;
mod lexer;

use std::env::args;
use std::fs;

use compiler::compile;
use interpreter::interpret;
use lexer::*;

fn main() {
    // ARG: int/comp | interpret or compile
    let arg1 = args().nth(1).expect("First argument missing");
    // ARG: <filename>
    let arg2 = args().nth(2).expect("Second <filename> argument missing");

    let to_interpret = if arg1  == "int" { true } else { false };
    let source = fs::read_to_string(arg2).expect("Could not read file");
    let (tokens, bracket_pairs) = lex(source);
    
    if to_interpret {
        interpret(tokens, bracket_pairs);
    } else {
        compile(tokens);
    }
}
