use std::io::{self, Write, stdout};
use std::env;
use std::fs;

fn get_filename() -> Option<String> {
	let args: Vec<String> = env::args().collect();

	if args.len() >= 2 {
		Some(args[1].clone())
	} else {
		None
	}
}

#[derive(Debug)]
enum BFTOKEN {
	INCPTR, // Increment the data pointer (to point to the next cell to the right)
	DECPTR, // Decrement the data pointer (to point to the next cell to the left)
	INCVAL, // Increment (increase by one) the byte at the data pointer
	DECVAL, // Decrement (decrease by one) the byte at the data pointer
	OUTPUT, // Output the byte at the data pointer
	GINPUT, // Accept one byte of input, storing its value in the byte at the data pointer
	STLOOP, // If byte at the data pointer is zero, jump instruction ptr forward to command after matching "]"
	ENLOOP  // If byte at the data pointer is nonzero, jump instruction ptr back to command after matching "["
}

fn exec_tokens(tokens: Vec<BFTOKEN>) {
	let mut array: Vec<i8> = vec![0; 30000];
	let mut ptr_location = 0; // "pointer" used to determine where we are on the array
	let mut inst_ptr = 0; // "pointer" used to determine which instruction we are running

	let mut loop_starts: Vec<usize> = Vec::new(); // used to keep track of the index where loops start

	loop {
		//println!("ptr_location: {} | pos_val: {}", ptr_location, array[ptr_location]);
		match tokens[inst_ptr] {
			BFTOKEN::INCPTR => {
				if ptr_location < array.len() - 1 {
					ptr_location += 1;
				}
			},
			BFTOKEN::DECPTR => {
				//if ptr_location > 0 {
					ptr_location -= 1;
				//}
			},
			BFTOKEN::INCVAL => {
				array[ptr_location] += 1;
			},
			BFTOKEN::DECVAL => {
				//if array[ptr_location] > 0 {
					array[ptr_location] -= 1;
				//}
			},
			BFTOKEN::OUTPUT => {
				print!("{}", (array[ptr_location] as u8) as char);
				stdout().flush().expect("Could not flush stdout")
			},
			BFTOKEN::GINPUT => {
				let mut input = String::new();
				io::stdin().read_line(&mut input).expect("Could not read stdin");
				
				let input: u8 = input.trim().parse().expect("Input was not a byte");
				
				array[ptr_location] = input as i8;
			},
			BFTOKEN::STLOOP => {
				loop_starts.push(inst_ptr)
			},
			BFTOKEN::ENLOOP => {
				if array[ptr_location] != 0 {                             // - 1 is added because +1 is added at the end of the loop anyways
					inst_ptr = (loop_starts[loop_starts.len() - 1] - 1).clone() // if last value was not 0, bring instruction pointer back to the matching [
				} else {
					loop_starts.pop(); // remove the start of the loop from the vector as it is finished
				}
			},
		}

		if inst_ptr < tokens.len() - 1 {
			inst_ptr += 1
		} else {
			break
		}
	}
}

fn parse_input(input: String) -> Vec<BFTOKEN> {
	let input = input
		.replace("\n", "")
		.replace("\r", "")
		.replace(" ", "")
		.replace("\t", "");
	let mut tokens = Vec::new();

	for item in input.chars() {
		match item {
			'>' => tokens.push(BFTOKEN::INCPTR),
			'<' => tokens.push(BFTOKEN::DECPTR),
			'+' => tokens.push(BFTOKEN::INCVAL),
			'-' => tokens.push(BFTOKEN::DECVAL),
			'.' => tokens.push(BFTOKEN::OUTPUT),
			',' => tokens.push(BFTOKEN::GINPUT),
			'[' => tokens.push(BFTOKEN::STLOOP),
			']' => tokens.push(BFTOKEN::ENLOOP),
			_   => {}
		}
	}

	tokens
}

fn main() {
	let filename = match get_filename() {
		Some(fname) => fname,
		None => panic!("No filename given")
	};

	let tokens = match fs::read_to_string(filename) {
		Ok(input_string) => parse_input(input_string),
		Err(_) => panic!("Could not read file!")
	};
	
    exec_tokens(tokens)
}
