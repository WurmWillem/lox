use std::fs;

use scanner::Scanner;

mod scanner;
mod token;
mod token_type;
mod expr;

fn main() {
    let mut lox = Lox::new();
    lox.run("file.lox");

}

struct Lox {
    //had_error: bool,
}

impl Lox {
    fn new() -> Self {
        //Self { had_error: false }
        Self {}
    }

    fn run(&mut self, source: &str) {
        let source = fs::read_to_string(source).unwrap();
        let source = source.to_string();
        let mut scanner = Scanner::new(source);

        let tokens = scanner.scan_tokens();
        for token in tokens {
            print!("{}", token.to_string());
        }
    }
}

pub fn error(line: usize, message: &str) {
    panic!("[line {}] Error: {}", line, message);
}
