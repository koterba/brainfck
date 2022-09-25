use std::io::{Write, stdout, stdin};
use std::collections::HashMap;

use crate::lexer::Token;

pub fn interpret(tokens: Vec<Token>, bracket_pairs: HashMap<usize, usize>) {
    let mut data: [i32; 30_000] = [0; 30_000]; // array used as the data store
    let mut ptr: usize = 0; // TODO: check if putting ptr in middle fixes stuff
    let mut pc: usize = 0;
    while pc < tokens.len() {
        match tokens[pc] {
            Token::INCPTR => {
                ptr += 1;
            }
            Token::DECPTR => {
                ptr -= 1;
            }
            Token::INCVAL => {
                data[ptr] += 1
            }
            Token::DECVAL => {
                data[ptr] -= 1
            }
            Token::INPUT => {
                let mut buffer = String::new();
                stdin().read_line(&mut buffer).expect("Could not read stdin");
                let mut res = buffer.bytes().nth(0).unwrap(); // get the first char of input as u8
                if res == 13 {res = 0}; // return should be 0, but is 13 in rust
                data[ptr] = res as i32;
            }
            Token::PRINT => {
                let byte = data[ptr] as u8; // cast i32 to u8;
                print!("{}", byte as char); // print u8 data as char
                stdout().flush().expect("") // no new line, so we need to flush buffer
            }
            Token::LOOPSTART => {
                if data[ptr] == 0 {
                    // if 0, stop loop and set program counter to the end of the loop
                    // which ends up being one after the end of the loop and the program counter
                    // gets incremented at the end of the main loop anyways
                    pc = *bracket_pairs.get(&pc).unwrap();
                }
            }
            Token::LOOPEND => {
                pc = *bracket_pairs.get(&pc).unwrap()-1; // if at the end of loop, go to beginning ALWAYS
            }
            _ => {}
        }
        pc += 1; // go to next instruction/token (increment program counter)
    }
}