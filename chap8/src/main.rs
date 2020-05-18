fn main() {

    let mut vector = vec![1,2,3,4,5,6,7,8,9];
    vector.pop();
    let mut check = vector[2];
    println!("\n{:#?}",check);

    let mut vector2: Vec<i32> = Vec::new();
    vector2 = vec![2];
    vector2.pop();
    vector2.push(10);
    vector2.push(20);
    vector2.push(30);
    println!("\n{:?}",vector2);

    // panicked karega in line 17
    // println!("\nfirst vector {:#?}",vector[100]); 

    println!("\nsecond vector first value {:#?}",vector2.get(0));

    match vector.get(1) {
        Some(value) => println!("{}",value),
        None => println!("Not present in the vector"),
    }

    for i in &mut vector {
         *i += 9;
        println!("{}",i);
    }
    
    let student = vec![school::branchNo(14),school::name(String::from("Civic School")), school::exist(true)];

    println!("{:?}",student);
 }

#[derive(Debug)]
enum school {
    branchNo(u32),
    name(String),
    exist(bool),
}
