use read_input::prelude::*;

fn main() {

    let number = input::<u32>()
                .msg("Enter numerator : ")
                .err("Please input positive integer")
                .get();

    let divisible = input::<u32>()
                .msg("Enter denominator : ")
                .err("Please input positive integer")
                .get();

    if number % divisible == 0 {
        println!("Number {} is completely divisible by {} ! ",number,divisible);
    }
    else {
        println!("Number {} is not completely divisible by {} ! ",number,divisible);        
    }
}
