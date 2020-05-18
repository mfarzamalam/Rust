use std::io::ErrorKind;
use std::fs::File;
use std::io::{self,Read};

fn main() {

    let breaking = vec![1,2,3,4];

    // breaking[59];    // we tweak code here in cargo.toml file as well. open it too see

    // just trying to open the file to see if it's present or not using match opetor

    // let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => panic!("file not found {:#?} ", error),
    // };


    // we try to open a file and if it failed then it create a new file.

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
                    ErrorKind::NotFound => match File::create("hello.txt") {
                        Ok(f) => f,
                        Err(error) => panic! ("Error in creating file"),
                    }

                    _ => {
                            panic!("You don't have permission to open a file");
                    }
        }
    };


    let c = File::open("another.rs");

    let c = match c {
        Ok(file) => file,
        Err(error) => match error.kind() {
                    ErrorKind::NotFound => match File::create("another.rs") {
                        Ok(c) => c,
                        Err(error) => panic!("there is a problem in creating a file"),
                    } 
                    _ => {
                        panic!("You don't have access to the file");
                    }
        }
    };


    // we use unwrap() method to shorten our code instead of using a whole bunch of match block AKA result enum

    let g = File::open("hello.txt").expect("hell.txt not found amigo");


    // Now we create a function to not only read file but also let us see what's inside it

    fn read_username_from_file() -> Result <String , io::Error> {
        let f = File::open("hello.txt");

        let mut f = match f {
            Ok(file)=> file,
            Err(error)=> return Err(error),
        };

        let mut s = String::new();

        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(error) => Err(error),
        }
    }

    // println!("{:#?}",read_username_from_file());


    // Now we create a function to not only read file but also let us see what's inside it (but in a short method)

    fn read_data_from_file() -> Result <String, io::Error> {
        let mut s = String::new();        
        let mut f = File::open("another.rs")?.read_to_string(&mut s); 
                    // question mark "?" represent the error feature and it return automatically if it's active
        
        // instead of writing this two line below, we could write it in one line. see above
        // let mut s = String::new();
        // f.read_to_string(&mut s)?;
        Ok(s)   // we should call the 
    }

    println!("{:#?}",read_data_from_file());

}
