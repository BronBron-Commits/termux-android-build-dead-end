use anyhow::Result;
use uchat_hybrid_signer::signer::HybridSigner;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: uchat-hybrid-signer <key.pem> <in.apk> <out.apk>");
        std::process::exit(1);
    }

    let key_path = &args[1];
    let in_apk = &args[2];
    let out_apk = &args[3];

    // Load key via the new API
    let signer = HybridSigner::load_key(key_path)?;

    // Perform signing
    signer.sign_apk(in_apk, out_apk)?;

    println!("Signed APK saved to {}", out_apk);
    Ok(())
}
