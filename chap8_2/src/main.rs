use std::collections::HashMap;

fn main() { 
    let mut map = HashMap::new();   // create new hashtable

    // inserting 3 values in the hashtable
    map.insert(String::from("blue"),10);
    map.insert(String::from("green"),20);
    map.insert(String::from("red"),30);

    println!("first {:?}", map);    // print that table
    
    map.insert(String::from("green"),500);  // overting in the already created hashtable
    
    println!("\nupdated value {:?}", map);  // print overwrited table

    map.entry(String::from("red")).or_insert(200);  // if the value is not their in the hastable then it added the value using or_insert function otherwise ignore it
    
    println!("\nChecking the value if it is present in the table or not {:#?}", map);
    
    
    // create two vector and joining'em in order to make a hashtable respectively

    let teams = vec![String::from("team 1"),String::from("team 2"),String::from("team 3"),String::from("team 4"),String::from("team 5")];
    let value = vec![011,021,031,041,051,061];

    let concat:HashMap<_,_> = teams.iter().zip(value.iter()).collect();     // call a function to join both of vector
    // let concat:HashMap<_,_> = value.iter().zip(teams.iter()).collect();  // swapping the line 27

    println!("\njoining both vector to form a hashmap {:?} ",concat);

    println!("\ngetting the key {:?}\n",concat.get(&String::from("team 1")));   // calling the key by giving their respective team

    
    // using the for loop to call all the data in hastable

    for (key,value) in &concat {
        println!("{} , {} ", key, value);
    }

    
    // To check the concepts of ownership in the light of hashtable
    
    let first = String::from("your dilemma");
    let second = String::from("is not ma headache");

    let mut map = HashMap::new();

    map.insert( &first, &second); // ownership concept apply here that's why we use ampersand
    println!("{:?}", map);    


    // updating a value based on old value 
    let mut each_words = HashMap::new();
    let my_string = "hello hello tatti fellow tatti fellow world very tatti fellow ";

    println!(" checking this string : {:?} " , my_string);

    for word in my_string.split_whitespace() {

        let count = each_words.entry(word).or_insert(0);

            *count += 1;

            // println!("{:#?}", each_words);
    }

    println!("{:#?}", each_words);
    
}
