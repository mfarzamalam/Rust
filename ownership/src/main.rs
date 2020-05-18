fn main() {
    let mut s = String::from("hello");
    s.push_str(", world ");
    println!("{}",s);

    let s1 = String::from("Forwarding ownership");
    borrow_owner(s1);

    let value = String::from("main Take and back value");
    println!("{}",take_and_back_value(value));

    let name = String::from("Pakistan bropaki");
    println!("{:#?}",cal_length(name));
}

fn borrow_owner(abc:String) {
    println!("{}",abc);
}

fn take_and_back_value (x:String) -> String {
    println!("From tabv function {}",x);
    x
}

fn cal_length (x:String) -> (String,String) {
    (x.len().to_string(),x)
}