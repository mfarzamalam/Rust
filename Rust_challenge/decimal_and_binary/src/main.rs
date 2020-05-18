use read_input::prelude::*;

fn main() {
    let decimal = 10;
    println!("Binary number of {} is {:b}",decimal,decimal);

    let binary = "1010";
    let convert = isize::from_str_radix(binary, 2).unwrap();
    println!("Decimal number of {} is {}", binary,convert);
}
