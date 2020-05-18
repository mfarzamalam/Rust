//////////////////////// FIRST PART /////////////////////////////////

// fn main() {
//     let s1 = "a";
//     let s2 = "h";

//     println!("{}",longest(s1,s2));
// }

// fn longest <'a> (x:&'a str , y:&'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     }
//     else {
//         y
//     }
// } 





//////////////////////// SECOND PART /////////////////////////////////




// #[derive(Debug)]
// struct ExcerptFromBooks <'a> {
//     part : &'a str,      //// lifetime hamesha refernce pe he lagega :) qk game hi ushii ka hai :))))))
// }

// fn main(){
//     let book = String::from("Today is friday, shaitan, is jelous outside");
//     let first_sentence = book.split(',').next().expect("unable to find the ',' ");

//     let important = ExcerptFromBooks {
//         part: first_sentence
//     };

//     println!("{:#?}",important);

// }




//////////////////////// THIRD PART /////////////////////////////////


// fn main() {
//     let c = "abc";
//     let item = c.as_bytes();

//     println!("The word we get from a string is : {}",get_words("Hello world"));

// }


// fn get_words(s:&str) -> &str {
//     let convert_into_bytes = s.as_bytes();   // we convert string into bytes
//     println!("{:#?}",convert_into_bytes); // print the value

//     for (i,&item) in convert_into_bytes.iter().enumerate() {     // using a for method to apply the condition
//         if item == b' ' {       // String break if it found a space 
//             return &s[0..i]         // returing the stirng before it found a space
//         }
        
//         println!("index: {}, alphabet: {}, ASCII: {}",i,item as u8 as char,item);
//                     // print the index value with alphabets and its ascii code side by side
    
//     }

//     &s[..]      // if it doesn't found the space then return the whole string
// }



///////////////////////// FOURTH PART ////////////////////////////

#[derive(Debug)]
struct ImportantExcerpt <'a> {
    part: &'a str,
}

impl <'a> ImportantExcerpt <'a> {
    fn new(&self) -> i32 {
        3
    }

    fn announce_return_part (&self, announcement: &str) -> &str {
        println!("{}",announcement);
        self.part
    }
}


fn main() {
    let novel = String::from("Call me from, Some years ago....");
    let first_sentence = novel.split('.')
                        .next()
                        .expect("couldn't find what you're looking");

    let i = ImportantExcerpt{ part: first_sentence };

    println!("{}",i.new());
    println!("{}",i.announce_return_part("hello world"));

}


