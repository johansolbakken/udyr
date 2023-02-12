use std::env;
use std::io;
use std::io::Write;

use token::Token;
use token::TokenType;
mod error;
mod scanner;
mod token;

#[derive(Debug, Clone)]
struct RunError {
    messages: Vec<String>,
}

fn run(source: &String) -> Result<(), RunError> {
    let mut scanner = scanner::Scanner::new(source);
    /*let tokens = scanner.scanTokens();

    tokens.foreach(|token, iter| {
        println!("{:?}", token);
    });*/

    let token = Token::new(TokenType::AND, source, source, 1);
    println!("{:?}", token);

    Ok(())
}

fn run_file(path: &String) {
    let source = std::fs::read_to_string(path).unwrap();
    run(&source);
}

fn run_prompt() {
    let mut line = String::new();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let bytes_read = std::io::stdin().read_line(&mut line).unwrap();
        if bytes_read == 1 && line == "\n" {
            break;
        }
        run(&line);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: udyr [script]")
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}
