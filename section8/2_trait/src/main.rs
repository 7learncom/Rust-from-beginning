use std::fmt::{Display, Formatter, Result};

struct Article {
    headline: String,
    author: String,
}

struct Tweet {
    username: String,
    content: String,
}

trait Summary {
    fn summarize(&self) -> String;

    fn default(&self) -> String {
        String::from("Default Content ... ")
    }
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }

    fn default(&self) -> String {
        String::from("I change Default behavior cause i can")
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

impl Tweet {}

impl Display for Tweet {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}, {})", self.username, self.content)
    }
}

fn notif(item: impl Summary, item2: impl Summary + Display) {
    println!("Breaking News! {}", item.summarize())
}

fn notify<T: Summary + Display>(item: T) {
    println!("Breaking News! {}", item.summarize())
}

fn return_with_trait() -> impl Summary {
    Tweet {
        username: String::from("pouya"),
        content: String::from("rust is !really easy"),
    }
}

fn main() {
    println!("Hello, world!");
}
