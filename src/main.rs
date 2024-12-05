use std::{fs, process::Command};

mod utils;
use utils::{find_in_paths, flush, get_path_string, get_pwd, push_concat, read_input};

struct ShellContext {
    pwd: String,
}

fn main() {
    let mut ctx = ShellContext { pwd: get_pwd() };
    // start repl
    loop {
        repl(&mut ctx);
    }
}

fn repl(ctx: &mut ShellContext) {
    print!("$ ");
    flush();
    let input = read_input();

    if input.len() == 0 {
        return;
    }

    let tokens = parse_shell_command_params(input.trim());
    let toks = tokens.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
    match toks[0] {
        "exit" => std::process::exit(0),
        "echo" => {
            println!("{}", toks[1..].join(" "));
        }
        "pwd" => {
            println!("{}", ctx.pwd);
        }
        "type" => {
            if toks.len() < 2 {
                eprintln!("No command provided!");
            } else {
                if is_builtin(toks[1]) {
                    println!("{} is a shell builtin", toks[1]);
                } else if let Some(addr) = find_in_paths(toks[1]) {
                    println!("{} is {}", toks[1], addr);
                } else {
                    println!("{}: not found", toks[1]);
                }
            }
        }
        "cd" => {
            ctx.pwd = change_pwd(ctx.pwd.as_str(), toks[1]);
        }
        _ => {
            if let Some(addr) = find_in_paths(toks[0]) {
                // it is an executable
                let output = Command::new(addr)
                    .args(&toks[1..])
                    .output()
                    .expect("Failed to execute it!");
                let out = String::from_utf8_lossy(&output.stdout.as_slice());
                print!("{}", out);
            } else {
                println!("{}: command not found", input.trim());
            }
        }
    }
    flush();
}

fn is_builtin(s: &str) -> bool {
    match s {
        "exit" | "echo" | "type" | "pwd" | "cd" => true,
        _ => false,
    }
}

fn change_pwd(old: &str, new: &str) -> String {
    let new = get_path_string(old, new);
    if !new.is_ok() {
        eprintln!("{}", new.unwrap_err());
        return old.to_string();
    }
    let new = new.unwrap();
    match fs::read_dir(new.as_str()) {
        Ok(_) => new,
        Err(_) => {
            println!("cd: {}: No such file or directory", new);
            old.to_string()
        }
    }
}

fn parse_shell_command_params(s: &str) -> Vec<String> {
    let mut toks: Vec<String> = Vec::new();
    let mut i = 0;
    let chars = s.chars().collect::<Vec<char>>();

    while i < chars.len() {
        match chars[i] {
            '\'' => {
                let mut qtemp = String::new();
                let mut j = i + 1;
                while j < chars.len() {
                    if chars[j] == '\'' {
                        break;
                    } else {
                        qtemp.push(chars[j]);
                    }
                    j += 1;
                }
                i = j;
                push_concat(&mut toks, qtemp.as_str());
            }
            '\"' => {
                let mut qtemp = String::new();
                let mut j = i + 1;
                while j < chars.len() {
                    if chars[j] == '\\' {
                        if j + 1 < chars.len() {
                            let cc = chars[j + 1];
                            if cc == '$' || cc == '`' || cc == '\"' || cc == '\\' {
                                qtemp.push(chars[j + 1]);
                            } else {
                                qtemp.push('\\');
                                qtemp.push(chars[j + 1]);
                            }
                            j += 1;
                        }
                    } else if chars[j] == '\"' {
                        break;
                    } else {
                        qtemp.push(chars[j]);
                    }
                    j += 1;
                }
                i = j;
                push_concat(&mut toks, qtemp.as_str());
            }
            _ => {
                let mut temp = String::new();
                while i < chars.len() {
                    if chars[i] == '\\' {
                        if i + 1 < chars.len() {
                            temp.push(chars[i + 1]);
                            i += 1;
                        }
                    } else if chars[i] == ' ' {
                        temp.push(chars[i]);
                        break;
                    } else {
                        temp.push(chars[i]);
                    }
                    i += 1;
                }
                if temp.len() > 0 {
                    push_concat(&mut toks, temp.as_str());
                }
            }
        }
        i += 1
    }
    toks.iter()
        .filter(|s| s.trim().len() > 0)
        .map(|s| s.trim().to_string())
        .collect::<Vec<String>>()
}
