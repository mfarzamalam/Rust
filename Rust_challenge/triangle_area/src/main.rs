use read_input::prelude::*;

fn main() {

let base = input::<f32>()
                .msg("Enter magnitude of Triangle base : ")
                .err("Please enter integer ")
                .get();

let height = input::<f32>()
                .msg("Enter magnitude of Triangle base : ")
                .err("Please enter integer ")
                .get();

let sum = 0.5 * base * height;

    println!("Area of a Triangle with Height {} and Base {} is {}",height,base,sum);
}