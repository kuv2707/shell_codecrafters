#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // start repl
    loop {
        repl();
    }
}

fn repl() {
    print!("$ ");
    io::stdout().flush().unwrap();
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    let toks = input.trim().split_whitespace().collect::<Vec<&str>>();
    match toks[0] {
        "exit" => std::process::exit(0),
        _ => {
            println!("{}: command not found", input.trim());
            io::stdout().flush().unwrap();
        }
    }
}
