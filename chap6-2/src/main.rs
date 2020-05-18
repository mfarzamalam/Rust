#[derive(Debug)]
enum order {
    quantity(u32),
    address{town:String, road:String, house:u32},
    ETA(u32,u32,u32),
    cancel,
}

impl order {
    fn call(&self) {
        println!("{:#?}" , self);
    }
}

fn sale(x:order) {

}

fn main () {
    let order1 = order::quantity(16);
    let order2 = order::address {
                   town: String::from("gulshan"), 
                   road: String::from("near karimabad"),
                   house: 774};
    let order3 = order::ETA(00,25,16);
    let order4 = order::cancel;

    println!("{:#?}" , order1);
    
    order2.call();

    println!("{:#?}",sale(order3));
}