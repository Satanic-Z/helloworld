//! Example: Basic usage of the HelloWorld library
//!
//! Run with: cargo run --example basic_usage

use helloworld::{greet, greet_formal};

fn main() {
    println!("=== Basic HelloWorld Examples ===");

    // Basic greeting
    println!("{}", greet("Rustacean"));

    // Formal greeting
    println!("{}", greet_formal("Developer", "Mr."));

    // Multiple greetings
    let names = vec!["Alice", "Bob", "Charlie"];
    for name in names {
        println!("{}", greet(name));
    }

    println!("=== End of Examples ===");
}
