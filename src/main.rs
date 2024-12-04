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
        "type" => {
            if toks.len() < 2 {
                eprintln!("No command provided!");
            } else {
                if is_builtin(toks[1]) {
                    println!("{} is a shell builtin", toks[1]);
                } else {
                    println!("{}: not found", toks[1]);
                }
                flushio();
            }
        }
        _ => {
            println!("{}: command not found", input.trim());
            flushio();
        }
    }
}

fn is_builtin(s: &str) -> bool {
    match s {
        "exit" | "echo" | "type" => true,
        _ => false,
    }
}

fn flushio() {
    io::stdout().flush().unwrap();
}
