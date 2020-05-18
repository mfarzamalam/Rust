#[derive(Debug)]

struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn square(size: u32 ) -> Rectangle {
        Rectangle {
            height: size,
            width: size,
        }
    }
}
use std::io;

fn main() {

    let mut s = String::new();
    println!("Please enter something");
    io::stdin().read_line(&mut s).expect("failed to read line");
    let s:u32 = s.trim().parse().expect("enter int");
    
    println!("you enter {}",s);

    println!("{:#?}",Rectangle::square(17));
}
