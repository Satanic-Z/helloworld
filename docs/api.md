# API Documentation

This document provides detailed API documentation for the HelloWorld library.

## Functions

### `greet(name: &str) -> String`

Generates a greeting message for the given name.

**Parameters:**
- `name`: The name to greet (string slice)

**Returns:**
- A greeting string in the format "Hello, {name}!"

**Example:**
```rust
let greeting = greet("World");
assert_eq!(greeting, "Hello, World!");
```

### `greet_formal(name: &str) -> String`

Generates a formal greeting message.

**Parameters:**
- `name`: The name to greet formally (string slice)

**Returns:**
- A formal greeting string in the format "Good day, {name}. How are you?"

**Example:**
```rust
let greeting = greet_formal("Sir");
assert_eq!(greeting, "Good day, Sir. How are you?");
```

## Command Line Usage

The binary accepts the following command line arguments:

```bash
# Basic greeting to "World"
./helloworld

# Greet a specific person
./helloworld Alice

# Formal greeting to "World"
./helloworld --formal

# Formal greeting to a specific person
./helloworld --formal Alice
```
