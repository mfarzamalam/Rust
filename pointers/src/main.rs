fn main() 
{
    let mut a = String::from("hello");

     {
        let c = &mut a;
                c.push_str("from c");
        println!("{}",c);
    }

    {
        let b = &mut a;
        b.push_str("from b");
        println!("{}",b);   
    }   


    let mut c = String::from("Coma");
    c.push_str("Estes");
    println!("{}",c);

}
