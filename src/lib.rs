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

/// Error types for greeting operations
#[derive(Debug, Clone, PartialEq)]
pub enum GreetingError {
    /// Empty name provided
    EmptyName,
    /// Invalid language code
    InvalidLanguage(String),
    /// Name too long (over 100 characters)
    NameTooLong(usize),
}

impl std::fmt::Display for GreetingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyName => write!(f, "Name cannot be empty"),
            Self::InvalidLanguage(lang) => write!(f, "Unsupported language code: {}", lang),
            Self::NameTooLong(len) => write!(f, "Name too long ({} characters), maximum is 100", len),
        }
    }
}

impl std::error::Error for GreetingError {}

/// Result type for greeting operations
pub type GreetingResult<T> = Result<T, GreetingError>;

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

/// Validates a name for greeting operations
///
/// # Arguments
///
/// * `name` - The name to validate
///
/// # Returns
///
/// `Ok(())` if valid, `Err(GreetingError)` if invalid
///
/// # Errors
///
/// * `GreetingError::EmptyName` - if name is empty or only whitespace
/// * `GreetingError::NameTooLong` - if name exceeds 100 characters
pub fn validate_name(name: &str) -> GreetingResult<()> {
    let trimmed = name.trim();
    
    if trimmed.is_empty() {
        return Err(GreetingError::EmptyName);
    }
    
    if trimmed.len() > 100 {
        return Err(GreetingError::NameTooLong(trimmed.len()));
    }
    
    Ok(())
}

/// Safely greets a person with error handling
///
/// # Arguments
///
/// * `name` - The name to greet
///
/// # Returns
///
/// `Ok(String)` with greeting message, or `Err(GreetingError)` if invalid
///
/// # Examples
///
/// ```
/// use helloworld::safe_greet;
///
/// let result = safe_greet("World");
/// assert!(result.is_ok());
/// assert_eq!(result.unwrap(), "Hello, World!");
///
/// let empty_result = safe_greet("");
/// assert!(empty_result.is_err());
/// ```
pub fn safe_greet(name: &str) -> GreetingResult<String> {
    validate_name(name)?;
    Ok(format!("Hello, {}!", name.trim()))
}

/// Safely greets a person with multilingual support and error handling
///
/// # Arguments
///
/// * `name` - The name to greet
/// * `language_code` - The language code
///
/// # Returns
///
/// `Ok(String)` with greeting message, or `Err(GreetingError)` if invalid
///
/// # Examples
///
/// ```
/// use helloworld::safe_greet_multilingual;
///
/// let result = safe_greet_multilingual("World", "en");
/// assert!(result.is_ok());
///
/// let empty_result = safe_greet_multilingual("", "en");
/// assert!(empty_result.is_err());
/// ```
pub fn safe_greet_multilingual(name: &str, language_code: &str) -> GreetingResult<String> {
    validate_name(name)?;
    
    let language = Language::from_code(language_code);
    let greeting = language.greeting_word();
    Ok(format!("{}, {}!", greeting, name.trim()))
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

    #[test]
    fn test_validate_name() {
        assert!(validate_name("Alice").is_ok());
        assert!(validate_name("  Bob  ").is_ok());
        
        assert!(matches!(validate_name(""), Err(GreetingError::EmptyName)));
        assert!(matches!(validate_name("   "), Err(GreetingError::EmptyName)));
        
        let long_name = "A".repeat(101);
        assert!(matches!(validate_name(&long_name), Err(GreetingError::NameTooLong(_))));
    }

    #[test]
    fn test_safe_greet() {
        assert_eq!(safe_greet("Alice").unwrap(), "Hello, Alice!");
        assert_eq!(safe_greet("  Bob  ").unwrap(), "Hello, Bob!");
        
        assert!(safe_greet("").is_err());
        assert!(safe_greet("   ").is_err());
    }

    #[test]
    fn test_safe_greet_multilingual() {
        assert_eq!(safe_greet_multilingual("World", "en").unwrap(), "Hello, World!");
        assert_eq!(safe_greet_multilingual("世界", "zh").unwrap(), "你好, 世界!");
        
        assert!(safe_greet_multilingual("", "en").is_err());
        assert!(safe_greet_multilingual("   ", "zh").is_err());
    }

    #[test]
    fn test_error_display() {
        let error = GreetingError::EmptyName;
        assert_eq!(error.to_string(), "Name cannot be empty");
        
        let error = GreetingError::InvalidLanguage("xyz".to_string());
        assert_eq!(error.to_string(), "Unsupported language code: xyz");
        
        let error = GreetingError::NameTooLong(150);
        assert_eq!(error.to_string(), "Name too long (150 characters), maximum is 100");
    }
}
