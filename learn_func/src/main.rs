fn main() {
    let name = "farzam";
    println!("your name is {} ", name);


    
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    another_function();
}

fn another_function() {
    println!("From function");
}
