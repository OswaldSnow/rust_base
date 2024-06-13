// lib.rs

// Define a module
pub mod utils {
    // Define a function
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

// Define a struct
pub struct Person {
    pub name: String,
    pub age: u32,
}

impl Person {
    // Define a method
    pub fn new(name: &str, age: u32) -> Self {
        Person {
            name: name.to_string(),
            age,
        }
    }
}