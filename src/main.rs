//! HelloWorld Application
//!
//! A simple command-line application that demonstrates Rust best practices.

use helloworld::{greet, greet_formal, greet_multilingual, get_supported_languages};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    match args.len() {
        1 => {
            // No arguments provided
            println!("{}", greet("World"));
        }
        2 => {
            if args[1] == "--help" || args[1] == "-h" {
                print_help(&args[0]);
            } else if args[1] == "--languages" {
                print_supported_languages();
            } else {
                // One argument provided (the name)
                println!("{}", greet(&args[1]));
            }
        }
        3 => {
            // Check if second argument is a language code
            if args[1].len() <= 3 && !args[1].contains(' ') {
                // Assume it's a language code: ./app zh Alice
                println!("{}", greet_multilingual(&args[2], &args[1]));
            } else {
                // Two arguments - use them as title and name
                println!("{}", greet_formal(&args[2], &args[1]));
            }
        }
        4 => {
            // Three arguments - language, title, name: ./app zh Dr. Smith
            println!("{}", greet_multilingual(&format!("{} {}", &args[2], &args[3]), &args[1]));
        }
        _ => {
            // Too many arguments - show usage
            print_help(&args[0]);
        }
    }
}

fn print_help(program_name: &str) {
    println!("HelloWorld CLI - A multilingual greeting application");
    println!();
    println!("USAGE:");
    println!("    {}                          # Hello, World!", program_name);
    println!("    {} <name>                   # Hello, <name>!", program_name);
    println!("    {} <title> <name>           # Good day, <title> <name>!", program_name);
    println!("    {} <lang> <name>            # Greeting in specified language", program_name);
    println!("    {} <lang> <title> <name>    # Formal greeting in specified language", program_name);
    println!();
    println!("OPTIONS:");
    println!("    -h, --help        Show this help message");
    println!("    --languages       Show supported language codes");
    println!();
    println!("LANGUAGE CODES:");
    println!("    en - English (default)");
    println!("    zh - Chinese");
    println!("    es - Spanish");
    println!("    fr - French");
    println!("    de - German");
    println!();
    println!("EXAMPLES:");
    println!("    {} Alice                    # Hello, Alice!", program_name);
    println!("    {} Dr. Smith               # Good day, Dr. Smith!", program_name);
    println!("    {} zh 李明                  # 你好, 李明!", program_name);
    println!("    {} es Señor García          # Hola, Señor García!", program_name);
}

fn print_supported_languages() {
    println!("Supported Languages:");
    let languages = get_supported_languages();
    for language in languages {
        let (code, name) = match language {
            helloworld::Language::English => ("en", "English"),
            helloworld::Language::Chinese => ("zh", "Chinese"),
            helloworld::Language::Spanish => ("es", "Spanish"),
            helloworld::Language::French => ("fr", "French"),
            helloworld::Language::German => ("de", "German"),
        };
        println!("  {} - {}", code, name);
    }
}
