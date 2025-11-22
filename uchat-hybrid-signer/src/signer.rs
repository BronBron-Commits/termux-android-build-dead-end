use anyhow::*;
use ring::rand::SystemRandom;
use ring::signature::{RsaKeyPair, RSA_PKCS1_SHA256};
use ring::digest;
use zip::{ZipWriter, write::FileOptions};
use std::fs::File;
use std::io::Write;

use crate::manifest::{make_manifest, make_cert_sf};

pub struct HybridSigner {
    keypair: RsaKeyPair,
}

impl HybridSigner {
    pub fn new(keypair: RsaKeyPair) -> Self {
        Self { keypair }
    }

    pub fn sign_apk(&self, apk_in: &str, apk_out: &str) -> Result<()> {
        let data = std::fs::read(apk_in)?;

        let digest_bytes = digest::digest(&digest::SHA256, &data);

        // Create signature buffer
        let mut sig = vec![0u8; self.keypair.public().modulus_len()];
        let rng = SystemRandom::new();

        self.keypair
            .sign(&RSA_PKCS1_SHA256, &rng, digest_bytes.as_ref(), &mut sig)
            .map_err(|_| anyhow!("RSA signing failed"))?;

        let manifest = make_manifest(digest_bytes.as_ref())?;
        let cert_sf = make_cert_sf(digest_bytes.as_ref())?;

        let file = File::create(apk_out)?;
        let mut zip = ZipWriter::new(file);
        let opts = FileOptions::default();

        zip.start_file("META-INF/MANIFEST.MF", opts)?;
        zip.write_all(&manifest)?;

        zip.start_file("META-INF/CERT.SF", opts)?;
        zip.write_all(&cert_sf)?;

        zip.start_file("META-INF/CERT.RSA", opts)?;
        zip.write_all(&sig)?;

        // append original classes.dex
        zip.start_file("classes.dex", opts)?;
        zip.write_all(&data)?;

        zip.finish()?;
        Ok(())
    }
}
