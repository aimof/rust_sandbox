extern crate getch;

use std::sync::mpsc::channel;
use std::thread::spawn;

fn main() {
    println!("If you want to stop, please press esc key!");
    let (sender, receiver) = channel();
    let mut i = 0;
    spawn(move || {
        loop {
            println!("{}", i);
            i = i + 1;
            if receiver.recv().unwrap() == true {
                break;
            } else {
                continue;
            }
        }
    });
    loop {
        let key = getch::Getch();

        if ch == 0x1b {
            sender.send(true).unwrap();
            break;
        }
    }
}