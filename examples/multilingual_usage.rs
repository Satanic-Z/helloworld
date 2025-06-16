//! Example demonstrating multilingual greeting functionality
//!
//! Run with: cargo run --example multilingual_usage

use helloworld::{greet_multilingual, get_supported_languages};

fn main() {
    println!("=== Multilingual Greeting Demo ===\n");

    // Demo different languages
    let names = vec![
        ("World", "en"),
        ("世界", "zh"), 
        ("Mundo", "es"),
        ("Monde", "fr"),
        ("Welt", "de"),
    ];

    println!("Greeting in different languages:");
    for (name, lang_code) in &names {
        let greeting = greet_multilingual(name, lang_code);
        println!("  {} ({}): {}", lang_code, name, greeting);
    }

    println!("\nSupported languages:");
    let languages = get_supported_languages();
    for language in languages {
        println!("  {:?}", language);
    }

    // Demo with unknown language code (defaults to English)
    println!("\nUnknown language code defaults to English:");
    let unknown_greeting = greet_multilingual("Friend", "unknown");
    println!("  {}", unknown_greeting);
}
