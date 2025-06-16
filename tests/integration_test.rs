//! Integration tests for the HelloWorld application

use helloworld::{greet, greet_formal};

#[test]
fn test_integration_greet() {
    let result = greet("Integration Test");
    assert_eq!(result, "Hello, Integration Test!");
}

#[test]
fn test_integration_greet_formal() {
    let result = greet_formal("Integration Test");
    assert_eq!(result, "Good day, Integration Test. How are you?");
}

#[test]
fn test_multiple_calls() {
    let greeting1 = greet("First");
    let greeting2 = greet("Second");

    assert_eq!(greeting1, "Hello, First!");
    assert_eq!(greeting2, "Hello, Second!");
    assert_ne!(greeting1, greeting2);
}
