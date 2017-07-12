use std::cmp;

fn main() {
    let x_slice: &[i64] = &[8, 1, 7, 4, 0, 2, 6, 9, 3, 5];
    let (x_min, x_max) = find_min_max(x_slice);
    println!("{}", x_min); // 0
    println!("{}", x_max); // 9
    let y_slice = &[15, -43, 30000, -12345, 8];
    let (y_min, y_max) = find_min_max(y_slice);
    println!("{}", y_min); // -12345
    println!("{}", y_max); // 30000

    let z_slice = &[0, 1, 2, 3, 4, 5, 6];
    let mut z_slice2 = example(z_slice);
    let (z_min, z_max) = find_min_max(z_slice);
}

fn find_min_max(x: &[i64]) -> (i64, i64) {
    let mut min: i64 = x[0];
    let mut max: i64 = x[0];
    let mut x_iter = x.iter();
    while let Some(i) = x_iter.next() {
        min = cmp::min(*i, min);
        max = cmp::max(*i, max);
    }
    return (min, max)
}

fn example(y: &[i8]) -> &[i8] {
    return y
}