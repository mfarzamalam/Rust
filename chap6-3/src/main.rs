// here we learn about erum ...... ohh shit no, i mean ENUM ...... Damn ! 

#[derive(Debug)] 
enum option <t> {
    some(t),
    none,
}

#[derive(Debug)]
enum coin {
    penny,
    nickel,
    dime,
    quarter(UsStates)
}

#[derive(Debug)]
enum UsStates {
    alaska,
    alabama,
}

fn main() {
    let someint = option::some(69);

    let somestr = option::some("hi, i am alpha and i only want one thing from you. My daughter");

    println!("{:#?}",someint);

    println!("{:#?}",somestr);
    
    let anotherint = Some(4);

    let anotherstr = Some("you track'em , we whack'em");

    let zero: Option <i64> = None;

    println!("{:#?}",anotherint);

    println!("{:#?}",anotherstr);

    println!("{:#?}",zero);

    let x = 4;
    let z = 5;
    let y= Some(x+z);
    // let sum = x + y;
    println!("{:#?}",y);

    println!("{:#?} ", check_coin(coin::quarter(UsStates::alabama)));

    println!("{:#?} ", check_option(Some(45)));
    println!("{:#?} ",check_option(None));
}

fn check_option (x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(T) => Some(T+1), 
    }
}

fn check_coin (coin: coin) -> u8 {
    match coin {
         coin::penny => {print!("Lucky lad, ya got penny "); 1},
         coin::nickel => 5,
         coin::dime => 10,
         coin::quarter(whatever) => {print!("{:#?} ", whatever); 25},
    }
}


