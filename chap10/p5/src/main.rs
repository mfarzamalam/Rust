#[derive(Debug)]
struct Points<T,U> {
    x: T,
    y: U,
}

// impl Points<f32> {
//     fn distance_from_origin(&self) -> f32 {
//         {self.x.powi(2) + self.y.powi(2).sqrt()}
//     }
// }

impl<T,U> Points<T,U> {
    fn mixup<V,W> (self, other: Points<V,W>) -> Points<T,W> {
        Points {
            x: self.x,
            y: other.y,
        }
    } 
}



use std::fmt::Display;
#[derive(Debug)]
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}


use aggregator::Summary;

// use std::fmt::Display;


fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let string_list = vec!["alpha","bravo","charlie"];

    let result = largest(&number_list);
    let result2 = largest(&string_list);
    // println!("The largest number is {}", result);
    // println!("The largest number is {}", result2);

    let p1 = Points { x: 5, y: 10.4 };
    let p2 = Points { x: "Hello", y: 'c'};

    let p3 = p1.mixup(p2);

    // println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let tweet = aggregator::Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    let news1 = aggregator::NewsArticle {
            headline: String::from("Todays mark"),
            location: String::from("new york"),
            author: String::from("buzolski"),
            content: String::from("Weather"),
    };
    
    // println!("1 new tweet: {:#?}", tweet.summarize());
    // println!("1 new tweet: {:#?}", news1.summarize());

    println!("1 new tweet: {:#?}", news1);
    
    // aggregator::notify(tweet);


    let pair_1 = Pair::new('1','2');
    // println!("{:?}",pair_1.cmp_display());

    {
        let r;
    
        {
            let x = 5;
            r = &x;
        }
    
        println!("r: {}", r);
    }
    


    // println!("{}",aggregator::returns_summarizable());
}


// this fucntion only comparing char and i32
fn largest<T: PartialOrd + Copy> (list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item
        }
    }

    largest
}


// If we don’t want to restrict the largest function to the types that implement the Copy trait, 
// we could specify that T has the trait bound Clone instead of Copy. 
// Then we could clone each value in the slice when we want the largest function to have ownership. 
// Using the clone function means we’re potentially making more heap allocations in the case of types 
// that own heap data like String, and heap allocations can be slow if we’re working with large amounts 
// of data.

// Another way we could implement largest is for the function to return a reference to a T value in the slice.
// If we change the return type to &T instead of T, thereby changing the body of the function to 
// return a reference, we wouldn’t need the Clone or Copy trait bounds and we could avoid heap allocations. 
// Try implementing these alternate solutions on your own!





