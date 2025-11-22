use anyhow::{Result, bail};
use std::fs::{read, File};
use std::io::Write;
use zip::{ZipWriter, write::FileOptions};
use crate::manifest::{generate_manifest, generate_cert_sf};
use ring::rand::SystemRandom;
use ring::signature::{RsaKeyPair, RSA_PKCS1_SHA256};
use ring::digest;

pub struct HybridSigner {
    keypair: RsaKeyPair,
}

impl HybridSigner {
    pub fn load_key(path: &str) -> Result<Self> {
        let pem_data = read(path)?;

        let keypair = match RsaKeyPair::from_pkcs8(&pem_data) {
            Ok(k) => k,
            Err(_) => match RsaKeyPair::from_der(&pem_data) {
                Ok(k2) => k2,
                Err(_) => bail!("Invalid RSA key: must be PKCS8 or DER"),
            },
        };

        Ok(Self { keypair })
    }

    pub fn sign_apk(&self, input: &str, output: &str) -> Result<()> {
        let data = read(input)?;

        let digest = digest::digest(&digest::SHA256, &data);
        let digest_bytes = digest.as_ref();

        let rng = SystemRandom::new();
        let mut sig = vec![0u8; self.keypair.public().modulus_len()];

        if self.keypair.sign(&RSA_PKCS1_SHA256, &rng, digest_bytes, &mut sig).is_err() {
            bail!("Failed to produce RSA signature");
        }

        let manifest = generate_manifest(&data)?;
        let cert_sf = generate_cert_sf(&data)?;

        let outfile = File::create(output)?;
        let mut writer = ZipWriter::new(outfile);
        let opts = FileOptions::default().compression_method(zip::CompressionMethod::Stored);

        writer.start_file("META-INF/MANIFEST.MF", opts)?;
        writer.write_all(manifest.as_bytes())?;

        writer.start_file("META-INF/CERT.SF", opts)?;
        writer.write_all(cert_sf.as_bytes())?;

        writer.start_file("META-INF/CERT.RSA", opts)?;
        writer.write_all(&sig)?;

        // Add classes.dex only (template APK is minimal)
        writer.start_file("classes.dex", opts)?;
        writer.write_all(&data)?;

        writer.finish()?;
        Ok(())
    }
}
