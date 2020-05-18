use read_input::prelude::*;

fn main() {

    let mut n = 5;
    let mut counter = 0;

    for num in (0..n).rev() {        
        counter += num + 1;
    }
    
    println!("sum of n positive integer till {} is {}",n,counter);
}