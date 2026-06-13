use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
const FILE_PATH: &str = "passwords.json";

#[derive(Serialize, Deserialize, Debug)]
struct Credential {
    website: String,
    username: String,
    password: String,
}

fn load_credentials() -> Vec<Credential> {
    let content = fs::read_to_string(FILE_PATH);

    match content {
        Ok(data) => serde_json::from_str(&data).unwrap_or_else(|_| Vec::new()),
        Err(_) => Vec::new(),
    }
}

fn save_credentials(credentials: &Vec<Credential>) {
    let json = serde_json::to_string_pretty(credentials).expect("Failed to serialize");

    fs::write(FILE_PATH, json).expect("Failed to save file");
}

fn add_credential(credentials: &mut Vec<Credential>) {
    let mut website = String::new();
    let mut username = String::new();
    let mut password = String::new();

    println!("Website:");
    io::stdin().read_line(&mut website).unwrap();

    println!("Username:");
    io::stdin().read_line(&mut username).unwrap();

    println!("Password:");
    io::stdin().read_line(&mut password).unwrap();

    let credential = Credential {
        website: website.trim().to_string(),
        username: username.trim().to_string(),
        password: password.trim().to_string(),
    };

    credentials.push(credential);

    save_credentials(credentials);

    println!("Credential saved.");
}

fn view_credentials(credentials: &Vec<Credential>) {
    if credentials.is_empty() {
        println!("No credentials found.");
        return;
    }

    for credential in credentials {
        println!("--------------------");
        println!("Website: {}", credential.website);
        println!("Username: {}", credential.username);
        println!("Password: {}", credential.password);
    }
}

fn delete_credential(credentials: &mut Vec<Credential>) {
    view_credentials(credentials);

    if credentials.is_empty() {
        return;
    }
    let mut input = String::new();

    println!("Enter the index to delete");

    io::stdin().read_line(&mut input).unwrap();

    let index: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input.");
            return;
        }
    };

    if index == 0 || index > credentials.len() {
        println!("Index out of range.");
        return;
    }
    credentials.remove(index - 1);

    save_credentials(credentials);

    println!("Credential deleted.");
}

fn main() {
    let mut credentials = load_credentials();

    loop {
        println!("\n==== PASSWORD MANAGER ====");
        println!("1. Add Credential");
        println!("2. View Credentials");
        println!("3. Delete Credential");
        println!("4. Exit");

        let mut choice = String::new();

        io::stdin().read_line(&mut choice).expect("Failed input");

        match choice.trim() {
            "1" => add_credential(&mut credentials),
            "2" => view_credentials(&credentials),
            "3" => delete_credential(&mut credentials),
            "4" => break,
            _ => println!("Invalid choice"),
        }
    }
}
