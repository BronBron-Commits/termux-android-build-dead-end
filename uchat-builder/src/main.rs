use std::env;
use std::path::PathBuf;
use std::process::{Command, exit};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: uchat-builder <path-to-apk-project>");
        exit(1);
    }

    let project_path = PathBuf::from(&args[1]);
    let apk_target = project_path.join("target").join("aarch64-linux-android").join("release").join("apk").join("app.apk");

    println!("[1] Building APK with cargo-apk...");
    let build_status = Command::new("cargo")
        .arg("apk")
        .arg("build")
        .arg("--release")
        .current_dir(&project_path)
        .status()
        .expect("Failed to run cargo apk build");

    if !build_status.success() {
        eprintln!("APK build failed");
        exit(1);
    }

    println!("[2] Aligning APK...");
    let aligned_path = project_path.join("app-aligned.apk");
    let align_status = Command::new("target/release/uchat-zipalign")
        .arg(&apk_target)
        .arg(&aligned_path)
        .status()
        .expect("Failed to run uchat-zipalign");

    if !align_status.success() {
        eprintln!("Zipalign failed");
        exit(1);
    }

    println!("[3] Signing APK...");
    let signed_path = project_path.join("app-signed.apk");
    let sign_status = Command::new("target/release/uchat-apksigner")
        .arg(&aligned_path)
        .arg(&signed_path)
        .status()
        .expect("Failed to run uchat-apksigner");

    if !sign_status.success() {
        eprintln!("Signing failed");
        exit(1);
    }

    println!("✅ APK built, aligned, and signed: {}", signed_path.display());
}
