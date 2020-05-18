use read_input::prelude::*;

fn main() {
    
    let radius = input::<f32>()
                 .msg("input something : ")
                 .err("Please use some floating point number")
                 .get();

    let square = radius * radius;
    let area = 3.14159 * square;

    println!("The area of radius {} is : {}",radius,area);

}
