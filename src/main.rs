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
        // return;
    }
}

fn read_input(input: &mut String) {
    let stdin = io::stdin();
    stdin.read_line(input).unwrap();
    // input.push_str("echo 'hello  world   hi    ,'");
}

fn repl(ctx: &mut ShellContext) {
    print!("$ ");
    flushio();
    let mut input = String::new();
    read_input(&mut input);

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
// assumption: path string doesn't end with /
fn get_path_string(curr: &str, next: &str) -> Result<String, String> {
    let mut curr_parts = curr
        .split("/")
        .map(|s| s.to_string())
        .collect::<VecDeque<String>>();
    let mut next_parts = next
        .split("/")
        .map(|s| s.to_string())
        .collect::<VecDeque<String>>();

    let mut newpath_parts: VecDeque<String> = VecDeque::new();

    match next_parts[0].as_str() {
        "" => return Ok(next.to_string()),
        "." => {
            next_parts.pop_front();
            newpath_parts.append(&mut curr_parts);
        }
        ".." => {
            curr_parts.pop_back();
            next_parts.pop_front();
            newpath_parts.append(&mut curr_parts);
        }
        "~" => match env::var("HOME") {
            Ok(value) => {
                curr_parts.clear();
                let mut val = value
                    .split("/")
                    .map(|s| s.to_string())
                    .collect::<VecDeque<String>>();
                next_parts.pop_front();
                newpath_parts.append(&mut val);
            }
            Err(e) => {
                return Err(format!("Not found HOME: {}", e));
            }
        },
        _ => {
            newpath_parts.append(&mut curr_parts);
        }
    }

    for dir in next_parts {
        match dir.as_str() {
            ".." => {
                if newpath_parts.pop_back() == None || newpath_parts.len() == 0 {
                    return Err("Tried going above root".to_string());
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
    let okval = newpath_parts
        .iter()
        .map(|s| s.as_str())
        .collect::<Vec<&str>>()
        .join("/")
        .to_string();
    Ok(okval)
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

fn parse_shell_command_params(s: &str) -> Vec<String> {
    let mut toks: Vec<String> = Vec::new();
    let mut i = 0;
    while i < s.chars().count() {
        let c = s.chars().nth(i).unwrap();
        match c {
            '\'' => {
                i += 1;
                let nxt_sq = s[i..].find('\'').expect("Please escape single quote");
                toks.push(s[i..(i + nxt_sq)].to_string());
                i += nxt_sq;
            }
            '\"' => {
                i += 1;
                // this method of finding closing " ignores escaped "
                let nxt_sq = s[i..].find('\"').expect("Please escape single quote");
                toks.push(process_double_quoted_str(&s[i..(i + nxt_sq)]));
                i += nxt_sq;
            }
            _ => {
                let nxt_ws = s[i..].find(' ').unwrap_or(s.len() - i);
                let st = &s[i..(i + nxt_ws)];
                if st.len() > 0 {
                    toks.push(st.to_string());
                }
                i = i + nxt_ws;
            }
        }
        i += 1;
    }
    toks
}

// this is far from perfect
fn process_double_quoted_str(s: &str) -> String {
    let mut ret = String::new();
    let mut backsls = false;
    for k in s.chars() {
        match k {
            '$' | '`' | '"' | '\\' => {
                if backsls {
                    ret.push(k);
                    backsls = false;
                } else {
                    // each will show their special behaviour
                    // $ is used to access env vars. TODO this
                    if k == '\\' {
                        backsls = true;
                    } else {
                        ret.push(k);
                    }
                }
            }
            _ => {
                if backsls {
                    backsls = false;
                    ret.push('\\');
                }
                ret.push(k);
            }
        }
    }
    ret
}
