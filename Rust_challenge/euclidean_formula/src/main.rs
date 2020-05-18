use read_input::prelude::*;

fn main() {

let x1 = input::<i64>()
                .msg("Enter Coordinate for x1 : ")
                .err("Please enter positive integer ")
                .get();

let x2 = input::<i64>()
                .msg("Enter Coordinate for x2 : ")
                .err("Please enter positive integer ")
                .get();

let y1 = input::<i64>()
                .msg("Enter Coordinate for y1 : ")
                .err("Please enter positive integer ")
                .get();

let y2 = input::<i64>()
                .msg("Enter Coordinate for y2 : ")
                .err("Please enter positive integer ")
                .get();

let step1 = x1 - y1;
let step2 = x2 - y2;
let Last = (step1*step1)+(step2*step2);
let result = f32::sqrt(Last as f32);

    println!("Distance between points ({}, {}) and ({}, {}) is {}", x1,x2,y1,y2,result);
}