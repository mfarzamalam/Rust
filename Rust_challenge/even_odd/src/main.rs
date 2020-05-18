use read_input::prelude::*;

fn main() {

    let number = input::<i64>()
                .msg("Enter any number to check even or odd : ")
                .err("Plase use integer value")
                .get();

    if number % 2 == 0 {
        println!("Number is even");
    }
    else {
        println!("Number is odd");
    }

}
