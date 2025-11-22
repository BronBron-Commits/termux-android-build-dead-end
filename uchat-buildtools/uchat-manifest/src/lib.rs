use anyhow::*;
use sha2::{Sha256, Digest};
use std::fmt::Write as _;
use base64::Engine;

pub fn hash_bytes(data: &[u8]) -> Vec<u8> {
    let mut h = Sha256::new();
    h.update(data);
    h.finalize().to_vec()
}

pub fn generate_manifest(entries: &[(&str, &[u8])]) -> Result<Vec<u8>> {
    let mut out = String::new();

    for (name, bytes) in entries {
        let digest = hash_bytes(bytes);
        let b64 = base64::engine::general_purpose::STANDARD.encode(digest);

        writeln!(out, "Name: {}", name)?;
        writeln!(out, "SHA-256-Digest: {}", b64)?;
        writeln!(out)?;
    }

    Ok(out.into_bytes())
}

pub fn generate_cert_sf(manifest_bytes: &[u8]) -> Result<Vec<u8>> {
    let digest = hash_bytes(manifest_bytes);
    let b64 = base64::engine::general_purpose::STANDARD.encode(digest);

    let mut out = String::new();
    writeln!(out, "Signature-Version: 1.0")?;
    writeln!(out, "SHA-256-Digest-Manifest: {}", b64)?;
    writeln!(out)?;

    Ok(out.into_bytes())
}
