use anyhow::*;
use ring::rand::SystemRandom;
use ring::signature::{RsaKeyPair, RSA_PKCS1_SHA256};
use std::fs;

pub struct RsaSigner {
    keypair: RsaKeyPair,
}

impl RsaSigner {
    pub fn load(path: &str) -> Result<Self> {
        let pem = fs::read(path)
            .with_context(|| format!("Failed to read RSA key at {}", path))?;

        let keypair = RsaKeyPair::from_pkcs8(&pem)
            .or_else(|_| RsaKeyPair::from_der(&pem))
            .context("Invalid RSA key: must be PKCS8 or DER")?;

        Ok(Self { keypair })
    }

    pub fn sign(&self, data: &[u8]) -> Result<Vec<u8>> {
        let mut digest = [0u8; 32];
        digest.copy_from_slice(
            ring::digest::digest(&ring::digest::SHA256, data).as_ref()
        );

        let mut sig = vec![0u8; self.keypair.public().modulus_len()];
        let rng = SystemRandom::new();

        self.keypair
            .sign(&RSA_PKCS1_SHA256, &rng, &digest, &mut sig)
            .context("RSA signing failed")?;

        Ok(sig)
    }
}
