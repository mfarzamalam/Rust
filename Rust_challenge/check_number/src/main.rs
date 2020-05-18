use read_input::prelude::*;

fn main() {

    let number = input::<i32>()
                .msg("Enter any integer to check : ")
                .err("Not integer : ")
                .get();

    if number > 0 {
        println!("Positve number entered");
    }
    else if number < 0 {
        println!("Negative number entered");
    }
    else if number == 0 {
        println!("Zero entered");
    }
}
