fn main() {
    
    // Tuple is multiple datatype values
    
    let student = (12,"Usama",80.5);
    println!("Age : {}",student.0);
    println!("Name : {}",student.1);
    println!("Percentile : {}",student.2);
   
    println!("\nDestructure");
    let (mut a,n,p) = student;
    println!("{}",a);
    println!("{}",n);
    println!("{}",p);    
     
    a = 45; // we make a mutable
    println!("mutable {}",a);

}
