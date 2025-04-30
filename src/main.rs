// Project: HPMC
// File: main.rs
use lazy_static::lazy_static;
use spin::Mutex;
use std::io::{self, Read, Write};
use std::fs::File;

lazy_static! {
    pub static ref STDIN: Mutex<String> = Mutex::new(String::new());
}

fn main() {
    print_prompt();

    // Main loop to keep the program running
    let stdin = io::stdin();
    for byte in stdin.bytes() {
        match byte {
            Ok(b) => key_handle(b as char),
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                break;
            }
        }
    }
}

pub fn print_prompt() {
    print!("HPMC> ");
    io::stdout().flush().expect("Failed to flush stdout");
}

pub fn key_handle(c: char) {
    let mut stdin = STDIN.lock();
    if c == '\n' {
        print!("\n");
        match stdin.as_str().trim() {
            "" => {}
            "install" => {
                println!("Entering install mode");
                let mut link = String::new();
                let mut package = String::new();
                print!("Enter the link to the package: ");
                io::stdout().flush().expect("Failed to flush stdout");
                io::stdin()
                    .read_line(&mut link)
                    .expect("Failed to read line");
                print!("Enter the package name: ");
                io::stdout().flush().expect("Failed to flush stdout");
                io::stdin()
                    .read_line(&mut package)
                    .expect("Failed to read line");
                download(link.trim(), package.trim());
                println!("Package downloaded successfully");
            }
            _ => {
                println!("Unknown command: {}", stdin.as_str().trim());
            }
        }
        stdin.clear();
        print_prompt();
    } else {
        if c == 0x08 as char {
            // Handle backspace
            if stdin.len() > 0 {
                stdin.pop();
                print!("\x08 \x08"); // Erase the last character on the terminal
                io::stdout().flush().expect("Failed to flush stdout");
            }
        } else {
            stdin.push(c);
            print!("{}", c);
            io::stdout().flush().expect("Failed to flush stdout");
        }
    }
}

fn download(link: &str, package: &str) {
    println!("Downloading from: {}", link);
    println!("Saving as: {}", package);
    // Uncomment the following lines to enable actual downloading
    /*
    let mut resp = reqwest::blocking::get(link).expect("request failed");
    let mut out = File::create(package).expect("failed to create file");
    io::copy(&mut resp, &mut out).expect("failed to copy content");
    print_prompt();
    */
}