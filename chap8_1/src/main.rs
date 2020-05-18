fn main() {

    let mut my = String::new();

    my = String::from("hello");
    my.push_str(" worl");
    my.push('d');

    println!("{}",my);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{} ",s1,s2,s3);
    println!("{}",s);

    let english = String::from("sup bro");
    println!("{}", &english[0..1]);

    for c in english.chars() {
        println!("{}",c);
    }

    for c in english.bytes() {
        println!("{}",c);
    }
}
