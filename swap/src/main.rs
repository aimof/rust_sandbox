fn main() {
    let a: i64 = 50;
    let b: i64 = 15;
    let (a, b)  = swap0(a, b);
    println!("a: {}, b: {}", a, b);
    let c: i64 = 50;
    let d: i64 = 15;
    let (c, d) = return_i32(c, d);
    println!("c: {}, d: {}", c, d);
}

// 一次変数を用意する方法。大体どんな型でも使える。
fn swap0(a: i64, b: i64) -> (i64, i64) {
    return (b, a)
}

fn return_i32(_: i64, _: i64) -> (i32, i32) {
    let int1: i32 = 67;
    let int2: i32 = 22;
    return (int1, int2)
}