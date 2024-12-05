use std::{collections::HashMap, fs, process::Command};

mod utils;
use utils::{
    find_in_paths, flush, get_path_string, get_pwd, parse_shell_command_params, read_input,
};

struct ShellContext {
    pwd: String,
    builtins: HashMap<String, fn(&mut ShellContext, &Vec<&str>) -> ()>,
}

fn main() {
    let mut ctx = ShellContext {
        pwd: get_pwd(),
        builtins: HashMap::new(),
    };

    ctx.builtins.insert("exit".to_string(), exit_builtin);
    ctx.builtins.insert("echo".to_string(), echo_builtin);
    ctx.builtins.insert("pwd".to_string(), pwd_builtin);
    ctx.builtins.insert("type".to_string(), type_builtin);
    ctx.builtins.insert("cd".to_string(), cd_builtin);

    // start repl
    loop {
        repl(&mut ctx);
    }
}

fn exit_builtin(_ctx: &mut ShellContext, _toks: &Vec<&str>) {
    std::process::exit(0);
}

fn echo_builtin(_ctx: &mut ShellContext, toks: &Vec<&str>) {
    println!("{}", toks[1..].join(" "));
}

fn pwd_builtin(ctx: &mut ShellContext, _toks: &Vec<&str>) {
    println!("{}", ctx.pwd);
}

fn type_builtin(ctx: &mut ShellContext, toks: &Vec<&str>) {
    if toks.len() < 2 {
        eprintln!("No command provided!");
    } else {
        if is_builtin(ctx, toks[1]) {
            println!("{} is a shell builtin", toks[1]);
        } else if let Some(addr) = find_in_paths(toks[1]) {
            println!("{} is {}", toks[1], addr);
        } else {
            println!("{}: not found", toks[1]);
        }
    }
}

fn cd_builtin(ctx: &mut ShellContext, toks: &Vec<&str>) {
    ctx.pwd = calc_new_pwd(ctx.pwd.as_str(), toks[1]);
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

    let exec = ctx.builtins.get(toks[0]);
    if exec.is_none() {
        exec_command(&toks, &input);
    } else {
        let exec = exec.unwrap();
        exec(ctx, &toks);
    }
    flush();
}

fn exec_command(toks: &Vec<&str>, input: &String) {
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

fn is_builtin(ctx: &ShellContext, s: &str) -> bool {
    ctx.builtins.get(s).is_some()
}

fn calc_new_pwd(old: &str, new: &str) -> String {
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
