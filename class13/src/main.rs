use stu;

use std::collections::HashMap;

fn main() {
    let mut students = HashMap::new();

    students.insert(01,("farzam","farzam123@gmail.com",26));
    students.insert(02,("usama","usama123@gmail.com",15));    

    for (key,value) in &students {
        println!(" {} {:#?} " ,key,value);
    }

    stu::anotherclass::student::girls();
}