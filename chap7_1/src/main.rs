use chap7_1::restaurant::breakfast;

fn main() {
    
    let mut meal = breakfast::new(String::from("juice"),String::from("brown bread"));
    meal.bread = "blue bread".to_string();
    println!("{:#?}",meal);
}
