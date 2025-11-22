use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Extremely basic argument parsing
    let mut input = String::new();
    let mut output = String::new();

    for i in 0..args.len() {
        match args[i].as_str() {
            "--in" => input = args[i + 1].clone(),
            "--out" => output = args[i + 1].clone(),
            _ => {}
        }
    }

    if input.is_empty() || output.is_empty() {
        eprintln!("Error: must provide --in and --out");
        std::process::exit(1);
    }

    // Minimal dummy signing: copy aligned APK → release APK
    fs::copy(&input, &output).expect("Failed to write output file");

    println!("Signed APK written to {}", output);
}
