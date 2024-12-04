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
    flushio();
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    let toks = input.trim().split_whitespace().collect::<Vec<&str>>();
    match toks[0] {
        "exit" => std::process::exit(0),
        "echo" => {
            println!("{}", toks[1..].join(" "));
            flushio();
        }
        _ => {
            println!("{}: command not found", input.trim());
            flushio();
        }
    }
}

fn flushio() {
    io::stdout().flush().unwrap();
}
