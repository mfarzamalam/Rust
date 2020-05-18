///////////////////////////  first part ////////////////////////////

// #[derive(Debug)]
// struct Points<T> {
//     x:T,
//     y:T
// }

// impl <T> Points<T> {
//     fn first(&self) -> &T {
//         &self.x
//     }
// }

// impl Points<f64> {
//     fn distance(&self) -> f64 {
//         let result = self.x.powi(2) + self.y.powi(2);

//         result.sqrt()
//     } 
// }

// fn main() {
//     let p1 = Points{x:10.0, y:20.0};

//     println!("{:#?}",p1.distance());
// }




//////////////////////// Second part /////////////////////////////




// #[derive(Debug)]
// struct Points<a,b,c> {
//     x:a,
//     y:b,
//     z:c
// }

// impl <a,b,c> Points<a,b,c> {
//     fn mixup <d,e,f> (self, other: Points<d,e,f>) -> Points<a,e,c> {
//         Points {
//             x:self.x,
//             y:other.y,
//             z:self.z,
//         }
//     }
// }

// fn main() {
//     let p1 = Points{x:"ayyy", y:"bheee" ,z:"ceee"};
//     let p2 = Points{x:"deee", y:"eee", z:"efff"};

//     let p3 = p1.mixup(p2);
//     println!("{:#?}",p3);
// }





/////////////////////////////// Third part /////////////////////////////




// #[derive(Debug)]
// struct Superman {
//     name:String
// }

// #[derive(Debug)]
// struct Spiderman {
//     name:String
// }

// #[derive(Debug)]
// struct Batman {
//     name:String
// }

// #[derive(Debug)]
// struct Trashman {
//     name:String
// }

// pub trait Power {
//     fn muscles(&self) -> u8 {
//         02
//     }
// }

// impl Power for Superman {
//     fn muscles(&self) -> u8 {
//         100
//     }
// }

// impl Power for Spiderman {
//     fn muscles(&self) -> u8 {
//         100
//     }
// }

// impl Power for Batman {
//     fn muscles(&self) -> u8 {
//         100
//     }
// }

// impl Power for Trashman {}

// fn main() {
//     let hero1 = Superman {
//         name: String::from("I am Superman"),
//     };

//     let hero2 = Spiderman {
//         name: String::from("I am Spiderman"),
//     };

//     let hero3 = Batman {
//         name: String::from("I am Batman"),
//     };

//     let hero4 = Trashman {
//         name: String::from("I am Trashman"),
//     };

//     println!("{:#?}",hero1);
//     println!("{:#?}",hero2);
//     println!("{:#?}",hero3);
//     println!("{:#?} \n",hero4);

//     println!("Superman muscles is {:#?} \n",hero1.muscles());
//     println!("Spiderman muscles is {:#?} \n ",hero2.muscles());
//     println!("Batman muslces is {:#?} \n ",hero3.muscles());
//     println!("Trashman muscles is {:#?} \n ",hero4.muscles());
// }






////////////////////////////// Fourth part /////////////////////////////





////////// Creating a twiiter clone algo

// #[derive(Debug)]
// struct Twitter {  //declaring twitter struct
//     username: String,
//     content: String,
// }

// pub trait Empty {  // creating default trait empty with two functions
//     fn default(&self) -> String {
//         format!("No tweet has been done yet")
//     }

//     fn checking_tweet_length(&self) -> String ;
// }

// impl Empty for Twitter {  // applying trait in our struct
//     fn default(&self) -> String {  // the fuction first check wether the tweet is under 150 words with whitespaces and print the username and tweet
//         let checking_length = self.content.len(); // converting Sting tweet into length
        
//         // checking the condfitions
//         if checking_length > 150 {
//             format!("Error! Your tweet has more than 150 words")
//         }else {
//             format!("Length: {}, {}: {}",self.checking_tweet_length(),self.username,self.content)
//         }
//     }

//     //created a fucntion to convert the tweet into length with string version
//     fn checking_tweet_length(&self) -> String {
//         let checking_length = self.content.len().to_string();

//         checking_length
//     }

// }

// fn main() {
//     let tweet_1 = Twitter {
//         username: String::from("@farzamAlam"),
//         content: String::from("Friendly has successfully thwarted the nefarious design of the enemy")
//     };

//     let tweet_2 = Twitter {
//         username: String::from("@aslamkhan"),
//         content: String::from("Its your boys who is creating the whole ruckus")
//     };

//     println!("{:#?}",tweet_1.default());
//     println!("{:#?}",tweet_2.default());
// }





//////////////////////////////// 5th part  //////////////////////////////////




// #[derive(Debug)]
// struct Book {
//     author: String,
//     name: String,
// }

// impl Book {
//     fn new(author:String, name:String) -> Book {
//         Book {
//             author,
//             name,
//         }
//     }
// }

// trait BookInformation(&self) -> {
//     fn info()
// }

// fn main() {
//     let data_1 = Book {
//         author: String::from("Farzam"),
//         name: String::from("Family life is fucked")
//     };

//     println!("{:#?}",data_1);
// }





/////////////////////////////////////// 6th part /////////////////////////////////






// #[derive(Debug)]
// struct driver {
//     Weight: String,
//     Liscence: String,
//     color: String
// }

// pub trait driverinfo {
//     fn car(&self) -> String;
//     fn truck(&self,Poetry: bool) -> String;
// }

// impl driverinfo for driver {
//     fn car(&self) -> String {
//         format!("Weight of carDriver: {} \n Liscence of car: {} \n color of car: {} " ,
//                 self.Weight, self.Liscence, self.color)
//     }

//     fn truck(&self, Poetry: bool) -> String {
//         format!("Weight of TruckDriver: {} \n Liscence of Truck: {} \n color of Truck: {} \n   Poetry : {} " ,
//                 self.Weight, self.Liscence, self.color, Poetry)
//     }

// }

// fn main() {
//     let d1 = driver{Weight:"10kg".to_string(),Liscence:"4 numbers".to_string(),color:"white".to_string()};
//     println!("{:#?}",d1.car());


//     let d2 = driver{Weight:"100kg".to_string(),Liscence:"10 numbers".to_string(),color:"yellow".to_string()};
//     println!("{:#?}",d2.truck(false));
// }