use std::collections::HashMap;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Token {
    INCPTR, // (>) increment the pointer
    DECPTR, // (<) decrement the pointer
    INCVAL, // (+) increment the value at pointer
    DECVAL, // (-) decrement the value at pointer
    INPUT,  // (,) accept one byte as input
    PRINT,  // (.) output the byte at pointer
    LOOPSTART,
    LOOPEND,
    INC(i32), // optimized version of INCVAL
    DEC(i32), // optimized version of DECVAL
    INVALID
}

pub fn lex(source: String) -> (Vec<Token>, HashMap<usize, usize>) {
    let unoptimized_tokens = generate_tokens(source);
    let tokens = optimize_tokens(unoptimized_tokens);
    let bracket_pairs = find_bracket_pairs(&tokens);

    (tokens, bracket_pairs)
}

fn generate_tokens(source: String) -> Vec<Token> {
    let mut tokens = Vec::new();
    for c in source.chars() {
        tokens.push(match c {
            '>' => Token::INCPTR,
            '<' => Token::DECPTR,
            '+' => Token::INCVAL,
            '-' => Token::DECVAL,
            ',' => Token::INPUT,
            '.' => Token::PRINT,
            '[' => Token::LOOPSTART,
            ']' => Token::LOOPEND,
             _  => Token::INVALID
        })
    }
    tokens
}

fn optimize_tokens(tokens: Vec<Token>) -> Vec<Token> {
    let mut pc: usize = 0; // 'program counter' to walk the vector
    let mut new_tokens = Vec::new();
    while pc < tokens.len()-1 {
        match tokens[pc] {
            Token::INCVAL => {
                let mut count = 1;
                while tokens[pc+1] == Token::INCVAL {
                    pc += 1;
                    count += 1;
                }
                new_tokens.push(Token::INC(count));
            },
            Token::DECVAL => {
                let mut count = 1;
                while tokens[pc+1] == Token::DECVAL {
                    pc += 1;
                    count += 1;
                }
                new_tokens.push(Token::DEC(count));
            },
            _ => new_tokens.push(tokens[pc])
        }
        pc += 1
    }
    new_tokens
}

// iterate through tokens and match opening and closing brackets
fn find_bracket_pairs(tokens: &Vec<Token>) -> HashMap<usize, usize> {
    let mut pairs: HashMap<usize, usize> = HashMap::new();
    let mut stack: Vec<usize> = Vec::new();
    for (index, token) in (tokens.iter()).enumerate() {
        match token {
            Token::LOOPSTART => {
                stack.push(index)
            },
            Token::LOOPEND   => {
                let loop_start = stack.pop().unwrap();
                pairs.insert(loop_start, index); // match start to end
                pairs.insert(index, loop_start);  // match end to start
            },
            _ => {}
        }
    }
    pairs
}