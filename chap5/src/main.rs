#[derive(Debug)]

struct Book {
    name: String,
    author: String,
    price: u32,
    availablity: bool,
}

// struct address {
//     name: &str,
//     location: &str,
//     availablity: bool,
// }

#[derive(Debug)]

struct rectangle {
    height: u32,
    width: u32,
}

fn main() {
    let b1 = Book {
        name:String::from("Book A"),
        author: String::from("Author A"),
        price: 500,
        availablity: false
    };

    // mutable banany sy hum struct ki koi bhi aik value ka data change krskty hain

    let mut b2 = Book {
        name:String::from("Book b"),
        author: String::from("Author B"),
        price: 100,
        availablity: true,
    };

    b2.name = String::from("author bb");

    
    println!("{:#?} , {:#?} ",b1,b2);
    
    println!("{:#?}" , build("dbsl".to_string(),"farzam".to_string(),3200));

    println!("{:#?}" , build_2("shorthand".to_string(),"farzam_2".to_string(),1200));

    // shorthand methood if the rest of the value of struct are same but string value should be given in all the new variables

    let b3 = Book {
        name: String::from("Book 3 "),
        author: String::from("Author B"),
        //availablity: false,
        ..b2
    };

        println!("{:#?} ",b3);


    // ownership of struct variable 

    // let ad1 = address {
    //     name: "farzam",
    //     location: "r48",
    //     availablity: false
    // };

    // println!("{:#?}",ad1);

    let rec1 = rectangle {
        height: 40,
        width: 40,
    };

    println!("\nThe area of rectangle is {:#?} \n",area(&rec1));

    println!("{:#?}",rec1);
    
}


// yahan function banaya hai aur return type book rkhi hai jo struct ka name hai

fn build (n1: String, a1: String, price: u32) -> Book {
    
    Book {
        name: n1,
        author: a1,
        price: price,
        availablity: false
    }
}


// Agr apky function ka parameter aur struct ki datatype ka name same hoo tw apko 2 baar likhnay ki zroort nahi hai
// iska short form field init shorthand use krna hta hai agr dono cheezain same hoon


fn build_2 (name: String, author: String, price: u32) -> Book {
    
    Book {
        name,
        author,
        price: price,
        availablity: true
    }
}

fn area (Rect : &rectangle) -> u32 {
    Rect.height + Rect.width
}