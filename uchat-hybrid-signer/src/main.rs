use anyhow::*;
use uchat_hybrid_signer::key::load_rsa_key;
use uchat_hybrid_signer::signer::HybridSigner;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: uchat-hybrid-signer <key.pem> <in.apk> <out.apk>");
        std::process::exit(1);
    }

    let key_path = &args[1];
    let apk_in = &args[2];
    let apk_out = &args[3];

    let key = load_rsa_key(key_path)?;
    let signer = HybridSigner::new(key);

    signer.sign_apk(apk_in, apk_out)?;

    println!("Signed APK saved to {}", apk_out);
    Ok(())
}
