#[derive(Debug)]
struct aeroplane {
    name:String,
    engine:String,
    country:String,
}

#[derive(Debug)]
struct point <T> {
    x:T,
    y:T,
}

// impl aeroplane {
//     fn new(name:String , engine:String ,country:String ) -> aeroplane {
//         aeroplane {
//             name,
//             engine,
//             country,
//         }
//     }

//     fn new_aero (&self) -> String {
//         let new_aeroplane_01 = format!("Name : {} Engine : {} Country : {} ", self.name, self.engine, self.country);
//         new_aeroplane_01

//     }

    pub trait information {
        fn info (&self) -> String {
            "No value".to_string()
        }
    }

    impl information for aeroplane {
        fn info(&self) -> String {
            let first = format! ("{}" , self.name);
            first
        }
    }

use std::fmt::Display;

fn main () {

    let plane = aeroplane {
        name: String::from("boing 747"),
        engine: String::from("A quality"),
        country: String::from("Pakistan"),
    };

    // println!("{:#?}",plane);
    // println!("{:#?}",plane.new_aero());

    // println!("{:#?}",plane.info());


    // let mut a1 = String::from("B");
    // let mut e1 = String::from("B");
    // let mut c1 = String::from("B");

    // let aeroplane1 = aeroplane::new(a1,e1,c1);

    // let aeroplane2 = aeroplane::new(String::from("c"),String::from("c"),String::from("c"));
    
    // println!("{:#?}",aeroplane2);


    
//////////////////////// Book code practice start ///////////////////////
 
    check_pass_value(32);
    check_pass_value("karachi");
    
    let v = vec![1,2,3,5,8,123,2,5,8];
   
    println!("{}",largest_number(&v));

    let values = point {x:8 , y:8};
    println!("{:#?}",values);
}

fn check_pass_value <T:Display> (x: T) {
    println!("{}",x);

}


fn largest_number(num:&[i32]) -> i32 {
    let mut largest = num[0];
    for &item in num.iter(){
           //1  >  2
        if item > largest {
            largest = item;
        }
    }

    largest
}

