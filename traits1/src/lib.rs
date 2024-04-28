use std::fmt::Display;

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify3(item: &impl Summary, item2: &impl Summary) {
    println!("Breaking news! {} and {}", item.summarize(), item.summarize());
}

pub fn notify4<T: Summary>(item: &T, item2: &T) {
    println!("Breaking news! {} and {}", item.summarize(), item.summarize());
}

pub fn notify5(item: &(impl Summary + Display)) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify6<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify7<T, U>(item: &T, item2: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Display
{
    33   
}

pub fn a_summarizable() -> impl Summary {
    MediumPost {}
}

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

pub struct MediumPost {

}

impl Summary for MediumPost {
    fn summarize_author(&self) -> String {
        String::from("by a random person")
    }
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        return String::new();
    }

    fn summarize_author(&self) -> String {
        format!("by {}", self.author)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}