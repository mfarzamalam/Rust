// First question of chap 8
// Given a list of integers, use a vector and return the mean (the average value), median (when sorted, the value in the middle position), and mode (the value that occurs most often; a hash map will be helpful here) of the list.
// find the mean, median and mode 

use std::collections::HashMap;

fn main () {
    let a = vec![5,5,5];
    let mut total = 0;
    let mut average = 0;

    for element in a.iter() {
        total += element;
        average += 1;
    }

    println!("Mean : {}", total/average);
    
    let mut map = HashMap::new();
    let mut mode = HashMap::new();

    for element in a.iter() {
        println!("{}",element);
        
        for 
    }

    map.insert(a,0);

    for word in map {
        let count = mode.entry(word).or_insert(0);
        *count += 1 ;
    }
    // println!("{:?}",mode);
}