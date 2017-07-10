use std::io;

fn main() {
    let i: i64 = 0;
    loop {
        println!("{}", i);
        let mut reader = io::stdin();
        let mut buffer: &str;g
        reader.read_exact(&mut buffer).unwrap();
        if String::from(buffer) == "\n" {

        }
    }
}
