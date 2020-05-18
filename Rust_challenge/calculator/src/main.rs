use std::io;
use read_input::prelude::*;

fn main() {

    loop {

    let mut first = String::new();
    io::stdin().read_line(&mut first);
    let first: f64 = first.trim().parse().unwrap(); // performing shadowing to convert the type 

    let mut operator = String::new();
    io::stdin().read_line(&mut operator);
    let operator: char = operator.trim().parse().unwrap(); // performing shadowing to convert the type 
    
    let mut second = String::new();
    io::stdin().read_line(&mut second);
    let second: f64 = second.trim().parse().unwrap(); // performing shadowing to convert the type 

        if first == 0.0 {
            break;
        }
        
        if operator == '+' {
            println!("{}", first + second);
        } else if operator == '-' {
            println!("{}", first - second);        
        } else if operator == '/' {
             println!("{}", first + second);        
        } else if operator == '*' {
            println!("{}", first * second);        
        } else if operator == '%' {
            println!("{}", first % second);        
        }
        }
}