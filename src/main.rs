// Project: HPMC
// File: main.rs
use lazy_static::lazy_static;
use spin::Mutex;
use std::io::{self, Read, Write};
use std::fs::File;
use std::path::Path;
use serde::Deserialize;

lazy_static! {
    pub static ref STDIN: Mutex<String> = Mutex::new(String::new());
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Package {
    name: String,
    link: String,
}

fn print_prompt() {
    print!("HPMC> ");
    io::stdout().flush().expect("Failed to flush stdout");
}

fn main() {
    print_prompt();
    let packagelistlocation = Path::new("src/packagelist.json");
    let packagelist = File::open(packagelistlocation)
        .expect("Failed to open packagelist.json");
    let packages: Vec<Package> = serde_json::from_reader(packagelist)
        .expect("error while reading or parsing");

    // Main loop to keep the program running
    let stdin = io::stdin();
    for byte in stdin.bytes() {
        match byte {
            Ok(b) => key_handle(b as char, &packages),
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                break;
            }
        }
    }
}

// Update key_handle to accept packages
pub fn key_handle(c: char, packages: &Vec<Package>) {
    let mut stdin = STDIN.lock();
    if c == '\n' {
        print!("\n");
        let input = stdin.as_str().trim();
        if input.is_empty() {
            // Do nothing
        } else if input.starts_with("install") {
            let parts: Vec<&str> = input.split_whitespace().collect();
            if parts.len() == 2 {
                let pkg_name = parts[1];
                if let Some(pkg) = packages.iter().find(|p| p.name == pkg_name) {
                    println!("Installing {} from {}", pkg.name, pkg.link);
                    download(&pkg.link, &format!("{}.zip", pkg.name));
                    println!("File downloaded successfully");
                } else {
                    println!("Package '{}' not found.", pkg_name);
                }
            } else {
                println!("Usage: install <package_name>");
            }
        } else {
            println!("Unknown command: {}", input);
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