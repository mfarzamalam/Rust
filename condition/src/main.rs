use std::io;

fn main() {
    
    let mut op = String::new(); // make a mutable string variable
    println!("Enter any operator");
    io::stdin().read_line(&mut op); //to get input from the user
    let op:i32 = op.trim().parse().unwrap();
    
    let num1=25;
    let num2=20;

   if op == 1 {
       println!("Sum is {}", num1 + num2);
   }
   else if op == 2 {
       println!("Sum is {}", num1 - num2);
   }
   else {
       println!("Invalid");
   }  
}