use std::env;
use std::io;
use std::io::Write;

fn run(line: &String) {
    println!("{}", line)
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
