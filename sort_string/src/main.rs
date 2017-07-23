use std::io::BufRead;

fn main() {
    loop {
        //let s = read_string();
        let s = "example".to_stirng;
        let exit = "exit".to_string();
        match s {
            exit => break,
            _ => println!("{}", sort_string(s))
        }
    }
}

fn read_string() -> String {
    let stdin = std::io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    return line;
}

fn sort_string(s: String) -> (&[i32]) {
    let mut s_iter = s.into_iter();
    let mut i32_slice: &[i32];
    while let Some(c) = s_iter.next() {
        i32_slice = c.parse().unwrap();
    }
    return i32_slice
}