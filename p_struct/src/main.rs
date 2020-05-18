#[derive(Debug)]

struct Book {
    name: String,
    author: String,
    price: u16,
    available: bool,
}

fn main() {

    let book_1 = Book {
                name: String::from("Book 1"),
                author: String::from("Author 1"),
                price: 400,
                available: false,
    };

    let mut book_2 = Book {
                name: String::from("Book 1"),
                author: String::from("Author 1"),
                price: 400,
                available: false,
    };

    book_2.name = String::from("Book 2");
    book_2.author = String::from("Author 2");

    println!("{:#?}, {:#?}",book_1,book_2);

    println!("{:#?}",my_book(String::from("book 3"), String::from("Author 3")));
}


fn my_book (name:String, author:String) -> Book {
    Book {
        name,
        author,
        price: 900,
        available: true,
    }
}
