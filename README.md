# Rust Password Generator CLI

A command-line password generator built with Rust that allows users to create strong and secure passwords based on custom requirements.

## Add Dependencies
Open Cargo.toml

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

## Project Structure

password_manager
│
├── Cargo.toml
├── passwords.json
│
└── src
    └── main.rs

## Features

* Generate secure random passwords
* Custom password length
* Uppercase letters support
* Lowercase letters support
* Numbers support
* Special characters support
* User-friendly CLI interface

## Skills Demonstrated

* Rust Ownership & Borrowing
* Functions & Modules
* Vectors and Strings
* Pattern Matching
* Error Handling
* External Crates (rand)
* Command Line Applications

## Why I Built This

This project is part of my Rust learning journey to strengthen problem-solving skills and gain hands-on experience with systems programming concepts while building practical applications.

