use std::io::Write;
use std::{collections::VecDeque, env, fs, io};

pub fn flush() {
    io::stdout().flush().unwrap();
}

pub fn get_pwd() -> String {
    match env::current_dir() {
        Ok(value) => value.into_os_string().to_string_lossy().to_string(),
        Err(e) => e.to_string(),
    }
}

pub fn read_input() -> String {
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut input).unwrap();
    return input;
}

pub fn find_in_paths(s: &str) -> Option<String> {
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

pub fn get_path_string(curr: &str, next: &str) -> Result<String, String> {
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

pub fn push_concat(toks: &mut Vec<String>, temp: &str) {
    let last = toks.pop();
    if last.is_none() {
        toks.push(temp.to_string());
        return;
    }
    let mut last = last.unwrap();
    if !last.ends_with(" ") {
        last.push_str(temp);
        toks.push(last);
    } else {
        toks.push(last);
        toks.push(temp.to_string());
    }
}
