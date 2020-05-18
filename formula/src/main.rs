// fn main() {
//     let t:(i32,i32,i32) = (23,43,54);
    
//     let r = (t.0+t.1+t.2)/2;

//     let s = (r*(r-t.0)*(r-t.1)*(r-t.2)) as f64;
    
//     let f_val = s.sqrt();

//     println!("{} {} {}",r,s,f_val);

// }

// struct student {
//     first_name:String,
//     last_name:String,    
//     Age:i32,
//     father_name:String,
//     single:bool,
//     height:f64,
// }

// impl student {
//     fn info(self) -> String {

//     }
// }

// struct Area {
//     a: i32,
//     b: i32,
//     c: i32,
// }

// impl Area {
//     fn calc(self) -> f64 {
//         let s = (self.a + self.b + self.c)/2;
//         let r = (s*(s-self.a)*(s-self.b)*(s-self.c)) as f64;
//         let f_v = r.sqrt();
//         f_v
//     }
// }

// fn main () {
//     let area_01 = Area{a:23,b:43,c:54};
//     println!("{}",area_01.calc());
// }


// struct Area {
//     l: i32,
//     b: i32,
// }

// impl Area {
//     fn calc(self) -> i32 {
//         let info = self.l*self.b;

//         info
//     }
// }

// fn main () {
//     let area_01 = Area{l:12,b:12};
//     println!("{}",area_01.calc());
// }

mod vechile;
use vechile::myVechile;

fn main () {
    let vechile_01 = myVechile{
        company:"Toyota".to_string(),
        speed:440,
        model:2001,
        color:"Yellow".to_string(),
        };

        // println!("{:?}",vechile_01);

        println!("{}",vechile_01.calc());
    }