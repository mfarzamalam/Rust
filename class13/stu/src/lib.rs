pub fn class() {
    pub fn student() {
        pub fn boys() {
            println!("four boys in the class");
        }
    }
}

pub mod anotherclass {
   pub mod student {
       pub fn girls() {
            println!("two girls in the class");
        }
    }
}