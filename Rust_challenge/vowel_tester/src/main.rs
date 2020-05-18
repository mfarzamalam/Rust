use read_input::prelude::*;

fn main() {

    let letter = input::<char>()
                .msg("Enter a character : ")
                .err("Character is a single alphabet doofus ")
                .get();

    match letter {
        'A'|'a' => println!("{} is Vowel",letter),
        'E'|'e' => println!("{} is Vowel",letter),
        'I'|'i' => println!("{} is Vowel",letter),
        'U'|'u' => println!("{} is Vowel",letter),
        'O'|'o' => println!("{} is Vowel",letter),
        _ => println!(" {} is not a vowel",letter),
    }
    
}
