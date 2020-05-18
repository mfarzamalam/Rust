    #[derive(Debug)]
    pub struct myVechile {
        pub company: String,
        pub speed: i32,
        pub model: i32,
        pub color: String,
    }


     impl myVechile {
        pub fn calc(self) -> String {
            let info = format!("Company name {},\nSpeed {},\nModel {}, Color {}",self.company,
            self.speed, self.model, self.color);

            info
        }
    }