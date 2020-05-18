fn main() {

    let (value,value_1) = square(2,9.1);
    println!("Square {} and cube {}",value,value_1);

}

fn square(x:u32, y:f64) -> (u32,f64) {
    let result = x*x;
    let resultSecond = y*y;
    
    (result,resultSecond)
}
