use std::process::Command;
use std::io::Write;
use std::fs;

use crate::lexer::Token;

fn gcc_compile(filename: &str) {
    let _gcc = Command::new("gcc")
        .arg(filename)
        .spawn()
        .expect("Could not run command");
}

pub fn compile(tokens: Vec<Token>) {
    let mut buffer = String::from("#include <stdio.h>\n\nint main()\n{\n\tchar array[30000] = {0}; char *ptr = array;\n");
    for token in tokens {
        match token {
            Token::INCPTR => {
                buffer.push_str("    ++ptr;\n");
            }
            Token::DECPTR => {
                buffer.push_str("    --ptr;\n");
            }
            Token::INCVAL => {
                buffer.push_str("    ++*ptr;\n");
            }
            Token::DECVAL => {
                buffer.push_str("    --*ptr;\n");
            }
            Token::INPUT => {
                buffer.push_str("    *ptr = getchar();\n");
            }
            Token::PRINT => {
                buffer.push_str("    putchar(*ptr);\n");
            }
            Token::LOOPSTART => {
                buffer.push_str("    while (*ptr)\n    {\n");
            }
            Token::LOOPEND => {
                buffer.push_str("    }\n");
            },
            Token::INC(x) => {
                buffer.push_str(&format!("    *ptr += {};\n", x));
            }
            _ => {}
        }
    }
    // close main
    buffer.push_str("\treturn 0;\n}\n");
    // create/overwrite output.c
    let mut file = fs::File::create("output.c").expect("Could not create a file");
    // turn string into u8 vec
    let byte: Vec<u8> = buffer.bytes().collect();
    // write bytes to file
    file.write(&byte).expect("Could not write to file");
    // compile file
    gcc_compile("output.c");
}