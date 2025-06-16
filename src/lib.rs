//! HelloWorld Library
//!
//! This library provides simple greeting functionality with multilingual support.
//!
//! # Examples
//!
//! ```
//! use helloworld::{greet, greet_multilingual};
//!
//! let greeting = greet("World");
//! assert_eq!(greeting, "Hello, World!");
//!
//! let chinese_greeting = greet_multilingual("世界", "zh");
//! assert_eq!(chinese_greeting, "你好, 世界!");
//! ```

// Clippy configuration via attributes
#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    missing_docs,
    rust_2018_idioms
)]
#![allow(
    clippy::module_name_repetitions,
    clippy::similar_names,
    clippy::must_use_candidate
)]
#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::unreachable,
    clippy::todo,
    clippy::unimplemented,
)]

/// Supported languages for multilingual greetings
#[derive(Debug, Clone, PartialEq)]
pub enum Language {
    /// English
    English,
    /// Chinese (Simplified)
    Chinese,
    /// Spanish
    Spanish,
    /// French
    French,
    /// German
    German,
}

impl Language {
    /// Convert a language code string to Language enum
    ///
    /// # Arguments
    ///
    /// * `code` - Language code (en, zh, es, fr, de)
    ///
    /// # Returns
    ///
    /// `Language` enum variant, defaults to English for unknown codes
    fn from_code(code: &str) -> Self {
        match code.to_lowercase().as_str() {
            "zh" | "chinese" => Self::Chinese,
            "es" | "spanish" => Self::Spanish,
            "fr" | "french" => Self::French,
            "de" | "german" => Self::German,
            _ => Self::English,
        }
    }

    /// Get the greeting word for this language
    fn greeting_word(&self) -> &'static str {
        match self {
            Self::English => "Hello",
            Self::Chinese => "你好",
            Self::Spanish => "Hola",
            Self::French => "Bonjour",
            Self::German => "Hallo",
        }
    }
}

/// Generates a greeting message for the given name.
///
/// This function takes a name and returns a greeting string.
/// It's the most basic greeting function in this library.
///
/// # Arguments
///
/// * `name` - A string slice that holds the name of the person to greet
///
/// # Returns
///
/// A `String` containing the greeting message
///
/// # Examples
///
/// ```
/// use helloworld::greet;
///
/// let greeting = greet("World");
/// assert_eq!(greeting, "Hello, World!");
/// ```
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

/// Generates a formal greeting message.
///
/// This function provides a more formal greeting by including
/// both a title and name.
///
/// # Arguments
///
/// * `name` - A string slice that holds the name of the person to greet
/// * `title` - A string slice that holds the title (e.g., "Dr.", "Mr.", "Ms.")
///
/// # Returns
///
/// A `String` containing the formal greeting message
///
/// # Examples
///
/// ```
/// use helloworld::greet_formal;
///
/// let greeting = greet_formal("Smith", "Dr.");
/// assert_eq!(greeting, "Good day, Dr. Smith!");
/// ```
pub fn greet_formal(name: &str, title: &str) -> String {
    format!("Good day, {} {}!", title, name)
}

/// Greets a person in multiple languages.
///
/// This function provides greeting in different languages based on
/// the language code provided.
///
/// # Arguments
///
/// * `name` - A string slice that holds the name of the person to greet
/// * `language_code` - A string slice representing the language code
///   (en, zh, es, fr, de). Defaults to English for unknown codes.
///
/// # Returns
///
/// A `String` containing the greeting message in the specified language
///
/// # Examples
///
/// ```
/// use helloworld::greet_multilingual;
///
/// let english = greet_multilingual("World", "en");
/// assert_eq!(english, "Hello, World!");
///
/// let chinese = greet_multilingual("世界", "zh");
/// assert_eq!(chinese, "你好, 世界!");
///
/// let spanish = greet_multilingual("Mundo", "es");
/// assert_eq!(spanish, "Hola, Mundo!");
/// ```
pub fn greet_multilingual(name: &str, language_code: &str) -> String {
    let language = Language::from_code(language_code);
    let greeting = language.greeting_word();
    format!("{}, {}!", greeting, name)
}

/// Get all supported languages
///
/// # Returns
///
/// A vector of all supported `Language` variants
///
/// # Examples
///
/// ```
/// use helloworld::get_supported_languages;
///
/// let languages = get_supported_languages();
/// assert!(languages.len() >= 5);
/// ```
pub fn get_supported_languages() -> Vec<Language> {
    vec![
        Language::English,
        Language::Chinese,
        Language::Spanish,
        Language::French,
        Language::German,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        assert_eq!(greet("World"), "Hello, World!");
        assert_eq!(greet("Rust"), "Hello, Rust!");
    }

    #[test]
    fn test_greet_formal() {
        assert_eq!(greet_formal("Smith", "Dr."), "Good day, Dr. Smith!");
        assert_eq!(greet_formal("Johnson", "Ms."), "Good day, Ms. Johnson!");
    }

    #[test]
    fn test_greet_multilingual() {
        assert_eq!(greet_multilingual("World", "en"), "Hello, World!");
        assert_eq!(greet_multilingual("世界", "zh"), "你好, 世界!");
        assert_eq!(greet_multilingual("Mundo", "es"), "Hola, Mundo!");
        assert_eq!(greet_multilingual("Monde", "fr"), "Bonjour, Monde!");
        assert_eq!(greet_multilingual("Welt", "de"), "Hallo, Welt!");
    }

    #[test]
    fn test_language_from_code() {
        assert_eq!(Language::from_code("zh"), Language::Chinese);
        assert_eq!(Language::from_code("es"), Language::Spanish);
        assert_eq!(Language::from_code("fr"), Language::French);
        assert_eq!(Language::from_code("de"), Language::German);
        assert_eq!(Language::from_code("unknown"), Language::English);
    }

    #[test]
    fn test_get_supported_languages() {
        let languages = get_supported_languages();
        assert_eq!(languages.len(), 5);
        assert!(languages.contains(&Language::English));
        assert!(languages.contains(&Language::Chinese));
    }

    #[test]
    fn test_empty_name() {
        assert_eq!(greet(""), "Hello, !");
        assert_eq!(greet_multilingual("", "zh"), "你好, !");
    }
}
