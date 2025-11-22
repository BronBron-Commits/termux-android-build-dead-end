use anyhow::Result;
use sha2::{Sha256, Digest};
use base64::{engine::general_purpose::STANDARD, Engine};

//
// Helper: returns the MANIFEST.MF content
//
pub fn generate_manifest(apk_bytes: &[u8]) -> Result<String> {
    let mut hasher = Sha256::new();
    hasher.update(apk_bytes);
    let digest = hasher.finalize();
    let b64 = STANDARD.encode(digest);

    let manifest = format!(
        "Manifest-Version: 1.0\nCreated-By: UChat Hybrid Signer\n\nName: classes.dex\nSHA-256-Digest: {}\n",
        b64
    );

    Ok(manifest)
}

//
// Helper: returns the CERT.SF content
//
pub fn generate_cert_sf(apk_bytes: &[u8]) -> Result<String> {
    let mut hasher = Sha256::new();
    hasher.update(apk_bytes);
    let digest = hasher.finalize();
    let b64 = STANDARD.encode(digest);

    let sf = format!(
        "Signature-Version: 1.0\nCreated-By: UChat Hybrid Signer\n\nName: classes.dex\nSHA-256-Digest: {}\n",
        b64
    );

    Ok(sf)
}
