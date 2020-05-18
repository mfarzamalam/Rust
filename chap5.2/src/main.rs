#[derive(Debug)]
struct car {
    name:String,
    speed: u32,
    seat: u32,
    model:u32,
    release:bool,
}

impl car {
    fn character (&self) -> car {
        
        (self.name,self.speed, self.seat,self.model, self.release)
    }
}

fn main() {
    let f = car {name:String::from("ferrari"), speed: 2000, seat: 4 , model: 2019 , release:true};

    let res = f.character();

    println!("{:#?}",res);
}