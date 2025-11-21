use std::io::{self, Write};
use std::process::Command;

fn main() {
    loop {
        println!("\n=== U-Chat Build Tools ===");
        println!("1. Build zipalign");
        println!("2. Build apksigner");
        println!("3. Build full workspace (release)");
        println!("4. Clean workspace");
        println!("5. Exit");

        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => run("cargo build --release -p uchat-zipalign"),
            "2" => run("cargo build --release -p uchat-apksigner"),
            "3" => run("cargo build --release"),
            "4" => run("cargo clean"),
            "5" => {
                println!("Goodbye.");
                break;
            }
            _ => println!("Invalid option."),
        }
    }
}

fn run(cmd: &str) {
    println!("\n> {}", cmd);
    match Command::new("sh").arg("-c").arg(cmd).status() {
        Ok(status) => {
            if status.success() {
                println!("✓ Done.");
            } else {
                println!("✗ Command exited with status: {:?}", status);
            }
        }
        Err(err) => {
            println!("✗ Failed to execute: {}", err);
        }
    }
}
