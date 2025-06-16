//! Integration tests for the HelloWorld application

use helloworld::{greet, greet_formal, greet_multilingual, get_supported_languages, Language};

#[test]
fn test_integration_greet() {
    let result = greet("Integration Test");
    assert_eq!(result, "Hello, Integration Test!");
}

#[test]
fn test_integration_greet_formal() {
    let result = greet_formal("Integration Test", "Dr.");
    assert_eq!(result, "Good day, Dr. Integration Test!");
}

#[test]
fn test_integration_greet_multilingual() {
    assert_eq!(greet_multilingual("Test", "en"), "Hello, Test!");
    assert_eq!(greet_multilingual("测试", "zh"), "你好, 测试!");
    assert_eq!(greet_multilingual("Prueba", "es"), "Hola, Prueba!");
}

#[test]
fn test_integration_supported_languages() {
    let languages = get_supported_languages();
    assert!(!languages.is_empty());
    assert!(languages.contains(&Language::English));
    assert!(languages.contains(&Language::Chinese));
    assert!(languages.contains(&Language::Spanish));
}

#[test]
fn test_integration_unknown_language_defaults_to_english() {
    let result = greet_multilingual("Test", "unknown_lang");
    assert_eq!(result, "Hello, Test!");
}

#[test]
fn test_multiple_calls() {
    let greeting1 = greet("First");
    let greeting2 = greet("Second");
    
    assert_eq!(greeting1, "Hello, First!");
    assert_eq!(greeting2, "Hello, Second!");
    assert_ne!(greeting1, greeting2);
}
