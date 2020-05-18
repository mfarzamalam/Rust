use std::io;

fn main() {
    
    let mut num = String::new();
    println!("Enter any number to do a factorial : ");
    io::stdin().read_line(&mut num);
    let num:i32 = num.trim().parse().unwrap();


    
}
