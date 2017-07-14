use std::io::BufRead;

fn main() {
    loop {
        let s = read_string();
        match s {
            "exit" => break;
            _ => sort_string(s);
        }
    }
}

fn read_string() -> String {
    let stdin = std::io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    return line;
}

fn sort_string(s: String) {
    let s_slice: &str = &s[..];
    let mut chars: Vec<char> = s_slice.chars().collect();
}