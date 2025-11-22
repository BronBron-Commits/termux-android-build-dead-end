use anyhow::*;
use base64::Engine;

pub fn make_manifest(digest: &[u8]) -> Result<Vec<u8>> {
    let b64 = base64::engine::general_purpose::STANDARD.encode(digest);
    let text = format!(
"Manifest-Version: 1.0
SHA-256-Digest: {}
", b64);
    Ok(text.into_bytes())
}

pub fn make_cert_sf(digest: &[u8]) -> Result<Vec<u8>> {
    let b64 = base64::engine::general_purpose::STANDARD.encode(digest);
    let text = format!(
"Signature-Version: 1.0
SHA-256-Digest: {}
", b64);
    Ok(text.into_bytes())
}
