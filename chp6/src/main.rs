
// START OF FIRST METHOD OF USING ENUM BY THE HELP OF STRUCT 

// #[derive(Debug)]
// enum Ipkind {
//     v4,
//     v6,
// }

// #[derive(Debug)]
// struct house {
//     ip: Ipkind,
//     ip_add: String,
//     address: String,
// }

// fn main() {
//     let customer1 = house {
//         address: String::from("r378"),
//         ip: Ipkind::v4,
//         ip_add: String::from("192.168.0.1"),
//     };

//     let customer2 = house {
//         address: String::from("r376"),
//         ip: Ipkind::v6,
//         ip_add: String::from("127.0.0.0.1")
//     };
    
//     println!("{:#?} , \n\n{:#?} ",customer1,customer2);
// }

// END OF FIRST METHOD OF USING ENUM BY THE HELP OF STRUCT 



// START OF SECOND METHOD OF USING ENUM DIRECTLY 

#[derive(Debug)]
enum iptype {
    v4(String),
    v6(u32,u32,u32,u32),
}

fn main () {
    let first = iptype::v4(String::from("a efff sting"));
    let second = iptype::v6(192,168,0,1);

    println!(" {:#?} " , first);
    println!(" {:#?} " , second);
}

