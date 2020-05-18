fn main() {
    // let mut num =4 ;
    // num = 5;
    // println!("value {} " , num) ;

    // println!("reference : {:p}" , &num) ; // pointer value keep changing, why ?


    // println!("Binary : {:b}" , &num) ;
    
    // println!("reference : {:x}" , &num) ;

    let mut num = 5;
    
    let r1 = &num as *const i32;
    let r2 = &mut num as *const i32;

    println!("{:p}",&num);

    unsafe {
        *r2 = 8;
        println!("{:?}",*r1);
        println!("{:?}",r2);
    }

}
