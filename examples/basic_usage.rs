//! Example: Basic usage of the HelloWorld library with error handling
//!
//! Run with: cargo run --example basic_usage

use helloworld::{greet, greet_formal, greet_multilingual};
use std::env;

fn main() {
    println!("=== Basic HelloWorld Examples with Error Handling ===");

    // Basic greeting
    println!("{}", greet("Rustacean"));

    // Formal greeting with input validation
    let title = "Mr.";
    let name = "Developer";
    if !name.trim().is_empty() && !title.trim().is_empty() {
        println!("{}", greet_formal(name, title));
    } else {
        eprintln!("Error: Name and title cannot be empty");
    }

    // Multiple greetings with error handling
    let names = vec!["Alice", "Bob", "Charlie", "", "David"];
    for name in names {
        if name.trim().is_empty() {
            eprintln!("Warning: Skipping empty name");
            continue;
        }
        println!("{}", greet(name));
    }

    // Multilingual greeting with error handling
    let test_cases = vec![
        ("World", "en"),
        ("世界", "zh"),
        ("", "es"),  // Empty name test
        ("Mundo", "invalid_lang"),  // Invalid language test
    ];

    println!("\n=== Multilingual Examples with Error Handling ===");
    for (name, lang) in test_cases {
        if name.trim().is_empty() {
            eprintln!("Error: Cannot greet empty name in language '{}'", lang);
            continue;
        }
        
        // Check if language is supported (simplified check)
        let supported_langs = ["en", "zh", "es", "fr", "de"];
        if !supported_langs.contains(&lang) {
            println!("Warning: '{}' not explicitly supported, using default (English)", lang);
        }
        
        println!("{}", greet_multilingual(name, lang));
    }

    // Command line argument handling with error recovery
    println!("\n=== Command Line Argument Handling ===");
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => println!("No additional arguments provided"),
        2 => {
            let arg = &args[1];
            if arg.trim().is_empty() {
                eprintln!("Error: Argument cannot be empty");
            } else {
                println!("Greeting argument: {}", greet(arg));
            }
        }
        _ => {
            println!("Multiple arguments provided, using first non-empty one:");
            for arg in &args[1..] {
                if !arg.trim().is_empty() {
                    println!("Using: {}", greet(arg));
                    break;
                }
            }
        }
    }

    println!("=== End of Examples ===");
}
