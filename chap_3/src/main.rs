fn main() {
    const my_age:u32 = 45;

    println!("age {}", my_age);

    let spaces = "BravoRomeoCharlio";
    println!("string {}", spaces);

    let spaces = spaces.len();
    println!("integer {}", spaces);

    let spaces = " calorie";
    println!("again string {}", spaces);

    let spaces = 'C';
    println!("A character {}", spaces);

    let num:u16 = 255;
    println!("{}", num);

    let myTup = (590,6.4,'a',"bonafied");
    println!("{}, {} , {} , {} ", myTup.0 , myTup.1, myTup.2, myTup.3);

    let (w,x,y,z,) = myTup;
    println!("{}", z);

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    println!("{}",months[0]);

    let element = months[];
    println!("{}",months);

}