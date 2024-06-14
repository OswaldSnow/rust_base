// lib.rs

// Define a module

//! # rust daily
//!
//! 一个自己学习的项目 一些无谓的代码
pub mod utils {
    //! utils mod
    // Define a function

    /// add method
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

// Define a struct
/// struct Person
pub struct Person {
    pub name: String,
    pub age: u32,
}

impl Person {
    // Define a method
    /// struct impl fu new
    pub fn new(name: &str, age: u32) -> Self {
        Person {
            name: name.to_string(),
            age,
        }
    }
}