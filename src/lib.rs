//! HelloWorld Library
//!
//! This library provides simple greeting functionality.
//!
//! # Examples
//!
//! ```
//! use helloworld::greet;
//!
//! let greeting = greet("World");
//! assert_eq!(greeting, "Hello, World!");
//! ```

/// Generates a greeting message for the given name.
///
/// # Arguments
///
/// * `name` - The name to greet
///
/// # Returns
///
/// A greeting string
///
/// # Examples
///
/// ```
/// use helloworld::greet;
///
/// let greeting = greet("Rust");
/// assert_eq!(greeting, "Hello, Rust!");
/// ```
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

/// Generates a formal greeting message.
///
/// # Arguments
///
/// * `name` - The name to greet formally
///
/// # Returns
///
/// A formal greeting string
pub fn greet_formal(name: &str) -> String {
    format!("Good day, {}. How are you?", name)
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
        assert_eq!(greet_formal("Sir"), "Good day, Sir. How are you?");
    }

    #[test]
    fn test_empty_name() {
        assert_eq!(greet(""), "Hello, !");
    }
}
