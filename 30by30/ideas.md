## 30 by 30 
Our very cool segment for posting tweets about:
- Tips
- Hacks  
- Do's & Don'ts!

This is a place where we can generate ideas!

## Tweet1
🔄 You can swap two values with creating a temp variable! 
#Rust30by30 #Rustc #Rustlang #Day1
```
    let a = 1;
    let b = 2;
    let (a, b) = (b, a);
    println!("{}, {}",a,b);
    //output: 2, 1
```

## Tweet2
🐞 Rust Hack: Use the `dbg!` macro to print variables and expressions during debugging. No more `println!`
#RustHacks #Debugging #Rust30by30 #Rustc #Rustlang #Day2

## Tweet 3
✅ DO: Use Rust's built-in testing framework to write unit tests and integration tests.
❌ DON'T: Skip testing, it's crucial for maintainable code! #RustTesting #TDD
#Rust30by30 #Rustc #Rustlang #Day3


## Tweet 4
🧩 Traits are not a method overloading, but a way to define methods that multiple types can implement, enabling polymorphism!
#Rust30by30 #Rustc #Rustlang #Day4
```
trait Printable {
    fn print(&self);
}

struct NewsArticle {
    title: String,
}

impl Printable for NewsArticle {
    fn print(&self) {
        println!("News Article: {}", self.title);
    }
}

struct BlogPost {
    title: String,
}

impl Printable for BlogPost {
    fn print(&self) {
        println!("Blog Post: {}", self.title);
    }
}
```
## Tweet 5
⚠️ Rust Tip: Use `.unwrap()` cautiously!
✅ DO: Use it in tests or when you're absolutely sure a `Result/Option` is `Some/Ok`.
❌ DON'T: Use in production code where errors are possible.
🔍 Better alternatives:
• ❓ `?` operator
• 🧩 `match`
• 🎯 `expect()`

📚 Read more: https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html
#RustErrorHandling #Rust30by30 #Rustc #Rustlang #Day5


## Tweet 6
🔧 Rust Trick
Use the `#[derive(Debug, Clone)]` attribute to automatically generate implementations for the Debug and Clone traits for your struct!

```
#[derive(Debug, Clone)]
struct Person {
    name: String,
    age: u32,
}
```
#RustTricks #Rust30by30 #Rustc #Rustlang #Day6

## Tweet 7
💡 Rust Tip #1: 
✅ DO: use `match` for pattern matching. 
❌ DON'T: rely on `if-else` for enum handling. 
#RustLang #30by30  #Rustc #Rustlang #Day7

```
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn get_light_action(light: TrafficLight) -> &'static str {
    match light {
        TrafficLight::Red => "Stop",
        TrafficLight::Yellow => "Caution",
        TrafficLight::Green => "Go",
    }
}
```

## Tweet 8
⚡ Rust Tip: Speed up compile times by using `cargo check` instead of `cargo build` during development. 
It skips building everything!
#RustTricks #Rust30by30 #Rustc #Rustlang #Day8

## Tweet 9
✅ DO: use `Option<T>` for values that might be absent.
❌ DON'T: use `null` pointers - Rust doesn't have them! 

```
struct animal {
    name: String,
    age: u32,
    fav_treats: Option<String>
}
```
#RustTricks #Rust30by30 #Rustc #Rustlang #Day9


## Tweet 10
✅ DO: use `&str` for string slices and String for owned strings. 
❌ DON'T: use `&String` as a function parameter. 
#RustTricks #Rust30by30 #Rustc #Rustlang #Day10
```
fn print_string(s: &str) {
    println!("{}", s);
}

fn main() {
    let my_string = String::from("Hello, world!");
    print_string(&my_string);
}
```

## Tweet 11
🎨 Rust Tip: Use `cargo fmt` to automatically format your code according to Rust style guidelines.
#RustTips #CodeStyle #Rust30by30 #Rustc #Rustlang #Day11

## Tweet 12
🔍 Rust Tip: Use `cargo expand` to see what your macros are actually generating!
✅ DO: Leverage this tool for debugging and understanding macro expansions.
❌ DON'T: Guess what your macros are doing—get the actual output!
#RustMacros #Rust30by30 #Rustc #Rustlang #Day12

## Tweet 13
🧪 Rust Hack: Use the `#[cfg(test)]` attribute to write inline tests in your source files.
#RustTesting #Rust30by30 #Rustc #Rustlang #Day13

## Tweet 14
🔍 Rust Tip: Use `cargo clippy` to catch common mistakes and improve your code quality.
#RustLinter #CodeQuality #Rust30by30 #Rustc #Rustlang #Day14

## Tweet 15
✅ DO: Use `Vec<T>` for growable arrays.
❌ DON'T: Use fixed-size arrays when you need dynamic sizing.
#RustCollections #Rust30by30 #Rustc #Rustlang #Day15

## Tweet 16
🔄 Rust Hack: Use the `std::mem::swap()` function to efficiently swap values.
#RustPerformance #Rust30by30 #Rustc #Rustlang #Day16

## Tweet 17
🔍 Rust Tip: Use the `std::fmt` module to create custom formatting!
#RustFormatting #Rust30by30 #Rustc #Rustlang #Day17
```
use std::fmt;
struct Person { name: String, age: u32 }
impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.age)
    }
}
```
## Tweet 18
⏱️ Rust Tip: Use `cargo bench` for benchmarking your code's performance.
#RustPerformance #Benchmarking #Rust30by30 #Rustc #Rustlang #Day18

## Tweet 19
🔍 Rust Hack: Use SQLX for type-safe SQL queries in Rust!
✅ DO: Leverage compile-time checked queries with `sqlx::query!`.
❌ DON'T: Risk runtime errors with string-based queries.
#RustDatabase #SQLX #Rust30by30 #Rustc #Rustlang #Day19
```
let users = sqlx::query!("SELECT * FROM users").fetch_all(db).await?;
```
## Tweet 20
⚠️ Rust Hack: Use the `#[must_use]` attribute to ensure important return values aren't ignored.
#RustSafety #Rust30by30 #Rustc #Rustlang #Day20
```
#[must_use]
fn important_function() -> Result<(), String> {
}
```

## Tweet 21
✅ DO: Use `std::collections::HashMap` for key-value storage.
❌ DON'T: Reinvent the wheel with custom data structures.
#RustCollections #Rust30by30 #Rustc #Rustlang #Day21

## Tweet 22
📚 Rust Tip: Use `cargo doc` to generate documentation for your project.
#RustDocs #Rust30by30 #Rustc #Rustlang #Day22

## Tweet 23
📦 Rust Tip: Embrace external crates! They're a strength, not a weakness.
✅ DO: Leverage the ecosystem for well-tested solutions.
❌ DON'T: Reinvent the wheel when great crates already exist.
#RustEcosystem #Crates #Rust30by30 #Rustc #Rustlang #Day23

## Tweet 24
🔁 Rust Hack: Use the `std::iter` module for powerful iterator operations.
#RustIterators #Rust30by30 #Rustc #Rustlang #Day24

## Tweet 25
🎨 Rust Tip: Use the `color_eyre` crate for customizable error reports!
✅ DO: Add by running `cargo add color_eyre` and then `cargo check` to see the magic!
❌ DON'T: Settle for plain, hard-to-read error messages.
#RustErrorHandling #Rust30by30 #Rustc #Rustlang #Day25

## Tweet 26
🔄 Rust Tip: Use `cargo update` to update your project's dependencies.
#RustDependencies #Rust30by30 #Rustc #Rustlang #Day26

## Tweet 27
✅ DO: Use `std::sync::Arc` for thread-safe reference counting.
❌ DON'T: Use `Rc` in multi-threaded contexts.
#RustConcurrency #Rust30by30 #Rustc #Rustlang #Day27

## Tweet 28
🏗️ Rust Hack: Use the `#[derive(Default)]` attribute for easy struct initialization.
#RustProductivity #Rust30by30 #Rustc #Rustlang #Day28

## Tweet 29
✅ DO: Leverage closures for flexible and reusable code. 💪
❌ DON'T: Avoid using closures when a simple function would suffice, as it can lead to unnecessary complexity.
```
fn add_one<F>(x: i32, f: F) -> i32
where F: Fn(i32) -> i32 {
    f(x) + 1
}
```
#RustCodeSnippets #Rust30by30 #Rustc #Rustlang #Day29

## Tweet 30
🌟 Rust Tip: Use the `axum` crate for building HTTP servers and handling requests easily!
#RustCommunity #Rust30by30 #Rustc #Rustlang #Day30
