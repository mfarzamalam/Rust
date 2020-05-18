/////////////////////// FIRST PART //////////////////////

// fn main() {
//     let first_closures = || {
//         println!("hello from closures");
//     };

//     first_closures();

    
//     let second_closures = |name| {
//         println!("Welcome {}",name);
//     };

//     second_closures("farzam");

// }


/////////////////////// SECOND PART ///////////////////////////


// fn main() {
//     let checking_prime_number = |num| {
//         for divisible in (1..num).rev() {
//             if num % divisible == 0 {
//                 println!("False");
//                 break;
//             } 
//             else {
//                 if divisible == 2 {
//                     println!("True");
//                     break;
//                 }
//             }
//         }
//     };
    
//     checking_prime_number(199);

// }




/////////////////////// THIRD PART ///////////////////////////



// use std::thread;
// use std::time::Duration;

// fn main() {

//     let simulated_user_specified_value = 10;
//     let simulated_random_number = 7;
    

//     simulated_expensive_calculation(4);

//     // let expensive_closure = |5| {
//     //     println!("calculating slowly..");
//     //     thread::sleep
//     // }

// }

// fn simulated_expensive_calculation(intensity: u32) -> u32 {
//     println!("Wait nigga, I'm calculating");
//     thread::sleep(Duration::from_secs(2));
//     intensity       
// }

// fn generate_workout(intensity: u32, random_number: u32) -> u32 {

//     // let expensive_workout = simulated_expensive_calculation(intensity);

//     if intensity < 25 {
//         println!("Today, do {} pushups!",simulated_expensive_calculation(intensity));
//         println!("Next, do {} situps!",simulated_expensive_calculation(intensity));
//     }
//     else {
//         if random_number == 3 {
//             println!("Take a break today, Remember to stay hydrated");
//         } else {
//             println!("Today run fo {} minutes",simulated_expensive_calculation(intensity));
//         }
//     }
// }




/////////////////////// FOURTH PART ///////////////////////////




use std::thread;
use std::time::Duration;

#[derive(Debug)]

struct Cacher<T> where T: Fn(u32) -> u32 {
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T> where T: Fn(u32) -> u32 {
    fn new(calculation:T) -> Cacher<T> {
        Cacher {
            calculation: calculation,
            value: None,
        }
    }

    fn value(&mut self, another_argument: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(another_argument);
                self.value = Some(v);
                v
            }
        }
    }
}

fn main() {

    generate_workout(1,11);
    
    // println!("{}",simulated_expensive_calculation(4));
}

    
fn generate_workout(intensity: u32, random_number: u32) {
    
    let mut expensive_closure = Cacher::new(|num| {
        println!("calculating nibba, wait ....");
        thread::sleep(Duration::from_secs(1));
    
        num
    });


    if intensity < 25 {
        println!("Today do {} pushups ", expensive_closure.value(intensity));
        println!("Next do {} situps ", expensive_closure.value(intensity));

    } 
    else {
       if random_number == 3 {
            println!("Take a break today and stay hydrated ! ");
        } 
        else {
            println!("Today run for {} minutes !", expensive_closure.value(intensity));
        }
    }
}