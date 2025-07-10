// Project: HPMC
// File: main.rs
// -----------------------------------------------------------------------------------
// [!] WARNING: HPMC is currently a prototype and DOES NOT WORK.
// -----------------------------------------------------------------------------------
// The current implementation does not properly select mirrors by priority, 
// does not parse direct URLs from the mirrorlist, and may not handle downloads correctly.
// Many features are incomplete or missing. Use this code for demonstration or development only.
// -----------------------------------------------------------------------------------
// TODOs:
// - Implement mirror selection by priority (1 = best, 3 = worst).
// - Parse and use direct URLs from the mirrorlist JSON.
// - Improve error handling and user feedback.
// - Add support for non-precompiled software.
// -----------------------------------------------------------------------------------
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
    url: String,
    country: String,
    protocol : String,
    priority : String
}

fn print_prompt() {
    print!("HPMC> ");
    io::stdout().flush().expect("Failed to flush stdout");
}

fn main() {
    print_prompt();
    let mirrorlistlocation = Path::new("src/mirrorlist.json");
    let mirrorlist = File::open(mirrorlistlocation)
        .expect("Failed to open mirrorlist.json");
    let packages: Vec<Package> = serde_json::from_reader(mirrorlist)
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

// Handles each key press and processes commands
fn key_handle(c: char, packages: &Vec<Package>) {
    let mut stdin = STDIN.lock();
    if c == '\n' {
        print!("\n");
        let input = stdin.as_str().trim();
        if input.is_empty() {
            // Do nothing for empty input
        } 

        // Handle 'install <package_name>' command
        else if input.starts_with("install") {
            let parts: Vec<&str> = input.split_whitespace().collect();
            if parts.len() == 2 {
                let pkg_name = parts[1];
                // Search for the package in the mirrorlist
                if let Some(pkg) = packages.iter().find(|p| p.name == pkg_name) {
                    println!("Installing {} from {}, country = {}", pkg.name, pkg.url, pkg.country);
                    download(&pkg.url, &format!("{}.zip", pkg.name));
                    println!("File downloaded successfully");
                } else {
                    println!("Package '{}' not found.", pkg_name);
                }
            } else {
                println!("Usage: install <package_name>");
            }

        // Handle exit/quit commands
        } else if input == "exit" || input == "quit" {
            println!("Exiting HPMC...");
            std::process::exit(0);

        // Handle help command
        } else if input == "help" {
            println!("Available commands:");
            println!("  install <package_name> - Install a package");
            println!("  exit or quit - Exit the program");
            println!("  help - Show this help message");

        // Handle clear command (clears terminal)
        } else if input == "clear" {
            print!("\x1B[2J\x1B[1;1H"); // Clear the terminal

        // Handle version command
        } else if input == "version" {
            println!("HPMC version 0.1.0");

        // Handle about command
        } else if input == "about" {
            println!("HPMC is a simple package manager for Rust.");

        // Handle unknown commands
        } else {
            println!("Unknown command: {}", input);
        }
        stdin.clear();
        print_prompt();
    } else {
        // Handle backspace key
        if c == 0x08 as char {
            if stdin.len() > 0 {
                stdin.pop();
                print!("\x08 \x08"); // Erase the last character on the terminal
                io::stdout().flush().expect("Failed to flush stdout");
            }
        } else {
            // Add character to input buffer and print it
            stdin.push(c);
            print!("{}", c);
            io::stdout().flush().expect("Failed to flush stdout");
        }
    }
}

/// Downloads a package from the given link and saves it as the given filename.
/// 
/// # Arguments
/// * `link` - The URL or identifier for the package source (currently assumed to be a GitHub repo path).
/// * `package` - The filename to save the downloaded file as.
///
/// TODO: 
/// - Update this function to support direct JSON URLs from the mirrorlist.
/// - Parse the `url` field from the JSON and use it directly if it's a full URL.
/// - If the URL is a GitHub repo path, keep the current behavior.
/// - Consider protocol handling (http/https) based on the JSON field.
fn download(link: &str, package: &str) {
    // If the link is a GitHub repo path, construct the download URL.
    // In the future, check if link is a full URL and use it directly.
    let url = format!("https://github.com/{}/archive/refs/heads/main.zip", link);
    println!("Downloading from: {}", url);

    // Send HTTP GET request to the constructed URL.
    let mut resp = reqwest::blocking::get(&url).expect("request failed");

    println!("Saving as: {}", package);

    // Create the output file and write the response content to it.
    let mut out = File::create(package).expect("failed to create file");
    io::copy(&mut resp, &mut out).expect("failed to copy content");
}