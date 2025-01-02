use std::env;

use colored::Colorize;
use rox::Rox;

mod environment;
mod expr;
mod interpreter;
mod rox;
mod parser;
mod scanner;
mod stmt;
mod token;
mod token_type;
mod value;

fn main() {
    let mut lox = Rox::new();

    let args: Vec<String> = env::args().collect();
    //env::set_var("RUST_BACKTRACE", "1");

    if args.len() == 1 {
        // run lox code from a file
        lox.run_file("file.lox");
    } else {
        // run lox code from a prompt
        let mut input = String::new();
        for i in 1..args.len() {
            let arg = format!("{} ", args[i]);
            input.push_str(&arg);
        }

        println!("{}", input);
        lox.run_prompt(input);
    }
}

pub fn crash(line: usize, message: &str) -> ! {
    let l = "[line ".blue();
    let i = "] Error: ".blue();
    let message = message.red();
    panic!("{}{}{}{}", l, line, i, message);
    //panic!("[line {}] Error: {}", line, message);
}
