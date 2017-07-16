use std::io::BufRead;

fn main() {
    loop {
        let s = read_string();
        let exit = "exit".to_string();
        match s {
            exit => break,
            _ => sort_String(s)
        }
    }
}

fn read_string() -> String {
    let stdin = std::io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    return line;
}

fn sort_string(s: String) -> () {

}