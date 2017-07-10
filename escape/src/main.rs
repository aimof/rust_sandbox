extern crate getch;

use getch::Getch;
use std::sync::mpsc::channel;
use std::thread::spawn;
use std::result::Result;
use std::io::Error;

fn main() {
    println!("If you want to stop, please press esc key!");
    let (sender, receiver) = channel();

    spawn(move || {
        loop{
            let g: Result<Getch, Error> = Getch::new();
            let key: Result<u8, Error> = Getch::getch(&g.ok().unwrap());
            const KEY_X: u8 = 120;

            let input_key = key.ok().unwrap();
            if input_key == KEY_X {
                sender.send(true).unwrap();
                break;
            }
        }
    });
    let mut i = 0;
    loop {
        println!("{}", i);
        i = i + 1;
        if receiver.try_recv().ok() != None {
            break;
        } else {
            continue;
        }
    }


}