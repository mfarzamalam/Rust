pub mod restaurant {

    #[derive(Debug)]
    pub struct breakfast {
        pub drink:String,
        pub bread:String,
        fruit:String,
    }

    impl breakfast {
        pub fn new(drink:String, bread:String) -> breakfast {
            breakfast {
                drink,
                bread,
                fruit:String::from("apple is compulsory lad"),
            }
        }
    }
}