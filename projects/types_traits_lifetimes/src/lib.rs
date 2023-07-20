#![allow(unused)]

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// types must implement std::cmp::PartialOrd and Copy
// largest that returns a copy
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

//no requirement for copy or clone trait
fn largest_ref<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &(list[0]);
    for item in list {
        if item > largest {
            largest = item
        }
    }
    largest
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    pub fn new(x: T, y: U) -> Point<T, U> {
        Point { x: x, y: y }
    }
}

impl<T, U> Point<T, U> {
    pub fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// Specialization that only applies to the concrete type i32
//impl Point<i32> {
//    pub fn new(x: i32, y: i32) -> Point<i32> {
//        Point { x: x, y: y }
//    }
//}

// Generics are implemented through Monomorphization, which incurs no additional
// runtime cost

#[allow(unused)]
pub fn make_points() {
    let one = Point { x: 10, y: 11 };
    let two = Point { x: 10.0, y: 11.0 };
    let three = Point::new(10, 11);
}

// Traits
// Due to the concept of coherence and the implementation of the orphan rule,
// (because the parent type is not present), we cannot implement external traits on external types.
// This ensures that third party code cannot break your existing implementation.

pub trait Summary {
    //fn summarize_author() -> String; //Required override
    fn summarize(&self) -> String {
        String::from("Read more...")
    }
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
    pub reply: String,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Trait bounds
pub fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize())
}

// Trait Boundary specification:
// fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 { ...
// Alternative:
// fn some_function<T, U>(t: T, u: U) -> i32
//     where T: Display + Clone,
//           U: Clone + Debug
// { ...

pub fn test_traits() {
    let tweet = Tweet {
        username: String::from("Stehfyn"),
        content: String::from("That was legitness!!"),
        reply: String::from("pog"),
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
}

// Trait bounds to conditionally implement methods

use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Pair<T> {
        Self { x, y }
    }
}

// Only if type T has implemented Display and PartialOrd
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest number is y = {}", self.y);
        }
    }
}

// Blanket implementation:
// Something similar in the std library ...
//impl<T: Display> ToString for T {
//
//}
// The effect is the following ...
// let s  = 3.to_string()
// Compile time checks retain "polymorphism" without
// any runtime perf or stability cost

// Lifetimes
// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime

fn longest_str<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

//use std::fmt:Display
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        test_traits();
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
