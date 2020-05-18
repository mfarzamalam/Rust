use read_input::prelude::*;

fn main() {

  let radius = input::<f32>()
              .msg("Enter radius of sphere : ")
              .err("Please use some floating point number")
              .get();
  
  let r = radius*radius*radius;

  let volume = 1.3333*3.14159*r;

  println!("Volume of sphere with Radius {} is {}",radius,volume);
}
