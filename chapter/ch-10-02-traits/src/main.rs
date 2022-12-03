use std::fmt::{Debug, Display};

fn main() {
    let tweet = Tweet {
        username: String::from("tom"),
        content: String::from("I'm happy today"),
        reply: false,
        retweet: false,
    };

    notify(&tweet);
    let new_article = NewsArticle {
        headline: String::from("moon find new mine"),
        location: String::from("moon"),
        author: String::from("tom"),
        content: String::from("a new mine find in moon"),
    };

    notify(&new_article);

    let pair = Pair::new(1, 2);
    pair.cmp_display();
}

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}:{}", self.username, self.content)
    }
}

pub fn notify(item: &impl Summary) {
    println!("News: {}", item.summarize());
}

pub fn notify2<T: Summary>(item1: &T, item2: &T) {
    println!("News: {},{}", item1.summarize(), item2.summarize());
}

pub fn notify3(item1: &impl Summary, item2: &impl Summary) {
    println!("News: {},{}", item1.summarize(), item2.summarize());
}

pub fn notify4(item: &(impl Summary + Display)) {
    println!("News: {}", item);
}

pub fn notify5<T: Summary + Debug>(item1: &T, item2: &T) {
    println!("News: {:?},{:?}", item1, item2);
}

pub fn some_fn<T, U>(t: &T, u: &U) -> f32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    0.
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x > self.y {
            println!("the largest num is x: {}", self.x);
        } else {
            println!("the largest num is y: {}", self.y);
        }
    }
}
