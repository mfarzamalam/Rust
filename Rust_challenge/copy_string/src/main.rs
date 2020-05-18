use read_input::prelude::*;

fn main() {

    let alpha = input::<String>().msg("Enter the string : ").get();
   
    let value = input::<u32>().msg("number of time it copy : ").get();

    let mut counter = 0;

    let result = loop {
            counter += 1;

            println!("{}", alpha);
    
            if counter == value {
                break;
            }
        };
}
