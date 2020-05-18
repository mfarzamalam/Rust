// #[derive(Debug)]
// struct Tweet {
//     username: String,
//     content: String,
// }

// #[derive(Debug)]
// struct Article {
//     author: String,
//     content: String
// }

// trait Summary {
//     fn info(&self) -> String;
// }

// impl Summary for Tweet {
//     fn info(&self) -> String {
//         format!("@{}: {}", self.username, self.content)
//     }
// }

// impl Summary for Article {
//     fn info(&self) -> String {
//         format!("@{}: {}", self.author, self.content)
//     }
// }

// // Using a trait bound syntax (for one data type)

// fn TweetNotify <T:Summary> (item1:T , item2:T) -> String {  
//     format!("{} , {}",item1.info(),item2.info())
// }


// // Using a impl trait syntax (for more than one data type)

// fn ArticleNotify (item1: impl Summary, item2: impl Summary) -> String {
//     format!("{} , {}",item1.info(),item2.info())
// }


// fn main() {
//     let tweet_1 = Tweet {
//         username: String::from("Farzam"),
//         content: String::from("This is a tweet one")
//     };

//     let tweet_2 = Tweet {
//         username: String::from("ALi"),
//         content: String::from("This is a tweet second")
//     };

//     let news_1 = Article {
//         author: String::from("Farzam"),
//         content: String::from("This is a news Article by Guardian")
//     };

//     let news_2 = Article {
//         author: String::from("Farzam"),
//         content: String::from("This is a news Article by Sputnix")
//     };

//     println!("{:#?}",TweetNotify(tweet_1,tweet_2) );
//     println!("{:#?}",ArticleNotify(news_1,news_2) );
// }





/////////////////////////// SECOND PART ////////////////////////////////////




use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug)]
struct Pair <T> {
    x:T,
    y: T,
}

impl <T> Pair <T> {
    fn new(x:T , y:T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl <T: Display + PartialOrd> Pair<T> {
    fn compare(&self) {
        if self.x > self.y {
            println!("x : {} is greater",self.x);
        }else {
            println!("y : {} is greater",self.y);
        }
    }
}


fn main() {
    
    let mut h1 = HashMap::new();
    h1.insert("blue",10);

    let mut h2 = HashMap::new();
    h2.insert("yellow",11);

    let pair_1 = Pair::new('a','z');
    let pair_2 = Pair::new(31,19);
    let pair_3 = Pair::new(h1,h2);

    pair_1.compare();
    pair_2.compare();
    println!("{:#?}",pair_3);


    let number_list = [0,1,2,33,4,5,6,7,8];
    // println!("{:#?}",largest(&number_list));  // Result of the largest function array
}



fn largest<T:PartialOrd + Copy>(list: &[T]) -> T {

    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}





//////////////////////////////// THIRD PART ///////////////////////////////////////



///// Checking the average value using generic

// use std::ops::Add;
// use std::ops::Div;

// fn averageG<T>(list: &[T]) -> T where T:Add<Output=T> + Div<Output=T> + Copy + Clone + From<i32>
// {
//     let mut sum = list[0];
//     let count = T::from(list.len() as i32);
//     for x in 0..list.len() {
//         sum = sum + list[x];
//     }

//     sum/count
// }


// fn main() {

//     ///// Checking the average value using local function

//     let a = [3,7,5,13,20,23,39,23,40,23,14,12,56,23,29];
//     let mut total = 0;
//     let mut average = 0;

//     for element in a.iter() {
//         total += element;
//         average += 1;
//     }

//     println!("Average is : {}", total/average);

//     println!("Average using generic function is : {}", averageG(&a));

//     let two = |n,m| {
//         (n + m) * 2
//     };

//     println!("{}",two(5,10));



//     // ASS link http://tiny.cc/IOTQ2S1
// }



/////////////////////////////// FOURTH PART ///////////////////////////////////////




// fn main() {
//     let r;

//     {
//         let c = 5;
//         r = &c;    // if we remove the reference (&) then it will work fine
//     }

//     println!("{}",r);
// }