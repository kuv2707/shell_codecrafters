#[allow(unused_imports)]
use std::io::{self, Write};
use std::{collections::VecDeque, env, fs, process::Command};

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
    flushio();
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    let toks = input.trim().split_whitespace().collect::<Vec<&str>>();
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
    flushio();
}

fn is_builtin(s: &str) -> bool {
    match s {
        "exit" | "echo" | "type" | "pwd" | "cd" => true,
        _ => false,
    }
}

fn find_in_paths(s: &str) -> Option<String> {
    match env::var("PATH") {
        Ok(value) => {
            let paths = value.split(":").collect::<Vec<&str>>();
            for path in paths {
                if path_contains_file(path, s) {
                    let ret = format!("{}/{}", path, s);
                    return Some(ret);
                }
            }
            None
        }
        Err(_e) => None,
    }
}

fn path_contains_file(path: &str, s: &str) -> bool {
    match fs::read_dir(path) {
        Ok(value) => {
            for entry in value {
                if !entry.is_ok() {
                    continue;
                }
                let entry: fs::DirEntry = entry.unwrap();
                if let Some(fname) = entry.file_name().to_str() {
                    if fname == s {
                        return true;
                    }
                }
            }
            false
        }
        Err(_e) => {
            // eprintln!("Could not read dir {}: {}", path, e);
            false
        }
    }
}

fn change_pwd(old: &str, new: &str) -> String {
    let new = get_path_string(old, new);
    match fs::read_dir(new.as_str()) {
        Ok(_) => new,
        Err(_) => {
            println!("cd: {}: No such file or directory", new);
            old.to_string()
        }
    }
}
// assumption: path string doesn't end with /
fn get_path_string(curr: &str, next: &str) -> String {
    let mut curr_parts = curr.split("/").collect::<VecDeque<&str>>();
    let mut next_parts = next.split("/").collect::<VecDeque<&str>>();

    let mut newpath_parts: VecDeque<&str> = VecDeque::new();

    match next_parts[0] {
        "" => return next.to_string(),
        "." => {
            next_parts.pop_front();
            newpath_parts.append(&mut curr_parts);
        }
        ".." => {
            curr_parts.pop_back();
            next_parts.pop_front();
            newpath_parts.append(&mut curr_parts);
        }
        _ => {
            newpath_parts.append(&mut curr_parts);
        }
    }

    for dir in next_parts {
        match dir {
            ".." => {
                if newpath_parts.pop_back() == None || newpath_parts.len() == 0 {
                    return String::new();
                }
            }
            _ => {
                if dir != "" {
                    newpath_parts.push_back(dir);
                } else {
                    break;
                }
            }
        }
    }
    newpath_parts
        .iter()
        .map(|s| *s)
        .collect::<Vec<&str>>()
        .join("/")
        .to_string()
}

fn get_pwd() -> String {
    match env::current_dir() {
        Ok(value) => value.into_os_string().to_string_lossy().to_string(),
        Err(e) => e.to_string(),
    }
}

fn flushio() {
    io::stdout().flush().unwrap();
}
