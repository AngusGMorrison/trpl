use std::fmt::Debug;
use std::fmt::Display;

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{} in {}", self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn trait_bound<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn trait_bound_syntax_sugar(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// first and second must have the same concrete type
pub fn trait_bound_multi_args<T: Summary>(first: &T, second: &T) {}

pub fn trait_bound_multi_traits<T: Summary + Display>(item: &T) {}
pub fn trait_bound_multi_traits_syntax_sugar(item: &(impl Summary + Display)) {}

pub fn trait_bound_many<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> () {}
pub fn trait_bound_where<T, U>(t: &T, u: &U) -> ()
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // Body
}

// impl syntax can only be used to return a value of a single concrete type.
// I.e. this function could not return a Tweet OR a NewsArticle.
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

fn largest_copy<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_clone<T: PartialOrd + Clone>(list: &[T]) -> T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest.clone()
}

fn largest_reference<T: PartialOrd + Clone>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// In these examples, Pair<T> always implements the `new` function, but it only
// implements the `cmp_display` method if T implements the `PartialOrd` and
// `Display` traits.
struct Pair<T> {
    x: T,
    y: T,
}

// Angle brackets after impl tell the compiler that T is a generic type rather
// than a concrete type.
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// Implement a trait for any type that implements another trait - blanket
// implementations.
impl<T: Display> ToString for T {}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins Win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Pengiuns once again are the best \
            hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize())
}
