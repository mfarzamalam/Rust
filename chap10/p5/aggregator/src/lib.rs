#[derive(Debug)]
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String{
        format!("Read More from {}...",self.summarize_author())
    }
}

pub trait Sam {
    fn sameer(&self) -> String {
        format!("hello from sam")
    }
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}",self.author)
    }

    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline,self.summarize_author(), self.location)
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}" , self.username)
    }
}

impl Sam for Tweet {}

pub fn notify(item: impl Sam) {
    println!("Breaking news! {}", item.sameer());
}


fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
