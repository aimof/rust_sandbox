
// 表示桁数の問題があるので50回で止めます
fn main() {
    let mut a: i64 = 1;
    println!("{}", a);
    let mut b: i64 = 1;
    println!("{}", b);
    for _ in 2..50 {
        println!("{}", a + b);
        let a_old = a;
        a = b;
        b = a_old + b;
    }
}