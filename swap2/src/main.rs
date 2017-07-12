use std::ptr;

fn main() {
    println!("Hello, world!");
}

fn my_swap<T>(x: &mut T, y: &mut T) {
    unsafe {
        let mut t: T = uninitialized();
        ptr::copy_nonoverlapping(&*x, &mut t, 1);
        ptr::copy_nonoverlapping(&*y, x, 1);
        ptr::copy_nonoverlapping(&t, y, 1);

        forget(t)
    }
}