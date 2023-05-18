pub mod aggregator;
use aggregator::{Summary, Tweet};

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    trait_compiles();
    life_time_on_functions();

    //lifetime on strcutures and other  datatypes
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    //Generic Type Parameters, Trait Bounds, and Lifetimes
    //Together
    use std::fmt::Display;

    //generic type "<.., T>";; trait bounds at whete the gen type implements Display trait
    //lifetime annotation to the lifetime of the shortest gen type  
    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    //using generic lifetimes so now the two inputs have a generic lifetime of a tied to them
    //so what ever the logical statement chooses does not affect them
    //its like tying ot variables to one pointer based on logic return
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn life_time_on_functions() {
    let r;
    {
        let x = 5;
        r = x;
    }
    println!("r: {}", r);

    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    //testing on different lifetimes => should take the smalles
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}

fn trait_compiles() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
    println!("{:?}", aggregator::returns_summarizable().summarize());
}
