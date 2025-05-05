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
                print!("Enter the repository owner and name in this format <owner>/<repo>: ");
                io::stdout().flush().expect("Failed to flush stdout");
                io::stdin()
                    .read_line(&mut link)
                    .expect("Failed to read line");
                print!("Enter the name of the downloaded file: ");
                io::stdout().flush().expect("Failed to flush stdout");
                io::stdin()
                    .read_line(&mut package)
                    .expect("Failed to read line");
                download(link.trim(), package.trim());
                println!("File downloaded successfully");
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
    let url = format!("https://github.com/{}/archive/refs/heads/main.zip", link);
    println!("Downloading from: {}", url);
    let mut resp = reqwest::blocking::get(&url).expect("request failed");
    println!("Saving as: {}", package);
    let mut out = File::create(package).expect("failed to create file");
    io::copy(&mut resp, &mut out).expect("failed to copy content");
    print_prompt();
}