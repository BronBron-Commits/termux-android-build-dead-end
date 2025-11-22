use anyhow::*;
use std::env;

fn usage() {
    println!("Usage:");
    println!("  uchat-signer <v1|v2> <key> <in.apk> <out.apk>");
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        usage();
        return Ok(());
    }

    let mode = &args[1];
    let key = std::fs::read(&args[2])?;
    let input = &args[3];
    let output = &args[4];

    match mode.as_str() {
        "v1" => {
            let manifest = uchat_v1sig::V1Manifest::simple_manifest();
            let data = std::fs::read(input)?;
            let sig = uchat_v1sig::V1Signer::sign(&data, &key)?;
            uchat_v1sig_assembler::assemble_v1(input, output, &manifest, &sig)?;
        }
        "v2" => {
            let data = std::fs::read(input)?;
            let v2_block = uchat_v2sig::V2Signer::sign_v2_block(&data, &key)?;
            uchat_v2sig::V2Signer::inject_v2_block(input, output, &v2_block)?;
        }
        _ => usage(),
    }

    Ok(())
}
