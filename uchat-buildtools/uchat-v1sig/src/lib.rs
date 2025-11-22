use anyhow::*;
use sha2::{Sha256, Digest};
use base64::engine::general_purpose::STANDARD as B64;
use base64::Engine;
use std::fs::File;
use std::io::Read;
use zip::ZipArchive;

pub struct ManifestResult {
    pub manifest: Vec<u8>,
    pub cert_sf: Vec<u8>,
    pub file_digests: Vec<(String, Vec<u8>)>,
}

fn sha256(data: &[u8]) -> Vec<u8> {
    let mut h = Sha256::new();
    h.update(data);
    h.finalize().to_vec()
}

pub fn generate_manifest(apk_path: &str) -> Result<ManifestResult> {
    let mut infile = File::open(apk_path)
        .with_context(|| format!("Failed to open APK: {}", apk_path))?;

    let mut zip = ZipArchive::new(&mut infile)
        .context("Failed to parse APK")?;

    let mut manifest_txt = String::new();
    manifest_txt.push_str("Manifest-Version: 1.0\n");

    let mut file_digests = Vec::new();

    for i in 0..zip.len() {
        let mut f = zip.by_index(i)?;
        let name = f.name().to_string();

        if name.starts_with("META-INF/") {
            continue;
        }

        let mut buf = Vec::new();
        f.read_to_end(&mut buf)?;

        let digest = sha256(&buf);
        let b64 = B64.encode(&digest);

        file_digests.push((name.clone(), digest.clone()));

        manifest_txt.push_str(&format!(
            "Name: {}\nSHA-256-Digest: {}\n\n",
            name, b64
        ));
    }

    Ok(ManifestResult {
        manifest: manifest_txt.as_bytes().to_vec(),
        cert_sf: vec![],
        file_digests,
    })
}

pub fn generate_cert_sf(manifest_bytes: &[u8], file_digests: &[(String, Vec<u8>)]) -> Result<Vec<u8>> {
    let manifest_digest = sha256(manifest_bytes);
    let manifest_b64 = B64.encode(&manifest_digest);

    let mut out = String::new();
    out.push_str("Signature-Version: 1.0\n");
    out.push_str(&format!("SHA-256-Digest-Manifest: {}\n\n", manifest_b64));

    for (name, dig) in file_digests {
        let b64 = B64.encode(dig);
        out.push_str(&format!(
            "Name: {}\nSHA-256-Digest: {}\n\n",
            name, b64
        ));
    }

    Ok(out.as_bytes().to_vec())
}
