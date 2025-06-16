//! HelloWorld Application
//!
//! A simple command-line application that demonstrates Rust best practices.

use helloworld::{greet, greet_formal};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            // No arguments provided
            println!("{}", greet("World"));
        }
        2 => {
            // One argument provided (the name)
            let name = &args[1];
            if name == "--formal" {
                println!("{}", greet_formal("World"));
            } else {
                println!("{}", greet(name));
            }
        }
        3 => {
            // Two arguments provided
            let flag = &args[1];
            let name = &args[2];
            if flag == "--formal" {
                println!("{}", greet_formal(name));
            } else {
                println!("{}", greet(name));
            }
        }
        _ => {
            eprintln!("Usage: {} [--formal] [name]", args[0]);
            std::process::exit(1);
        }
    }
}
