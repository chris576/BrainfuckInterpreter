use std::env;
use std::io::{ self, BufRead };

mod parser;
mod interpreter;

fn main() {
    let _args: Vec<String> = env::args().collect();
    let stdin = io::stdin();
    let mut byte_array: [u8; 10000] = [0; 10000];
    for line in stdin.lock().lines() {
        let command_string = line.unwrap();
        let is_valid = parser::is_valid(&command_string);
        if is_valid {
            let commands = parser::parse(&command_string);
            interpreter::run(&mut byte_array, &commands, 0, 0, None);
        }
    }
}
