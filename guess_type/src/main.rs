use std::mem::swap;

fn main() {
    let mut a = 1;
    let mut b = 2;
    swap(&mut a, &mut b);
    println!("{}", a);
    println!("{}", b);

    let c: i8 = 1;
    println!("{}", a == c);

    let d: i64 = 1;
    println!("{}", b == d); // error
}