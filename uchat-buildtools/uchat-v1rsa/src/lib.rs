use anyhow::*;
use rsa::pkcs1::DecodeRsaPrivateKey;
use rsa::{RsaPrivateKey, pkcs1v15::SigningKey};
use sha2::{Sha256, Digest};

pub struct V1RsaSigner {
    key: RsaPrivateKey,
}

impl V1RsaSigner {
    pub fn load_key(path: &str) -> Result<Self> {
        let pem = std::fs::read(path)
            .with_context(|| format!("Unable to read RSA key: {}", path))?;

        let key = RsaPrivateKey::from_pkcs1_pem(&String::from_utf8(pem)?)
            .context("Invalid PKCS1 RSA key")?;

        Ok(Self { key })
    }

    pub fn sign_sha256_der(&self, data: &[u8]) -> Result<Vec<u8>> {
        let digest = Sha256::digest(data);

        let signing_key = SigningKey::<Sha256>::new(self.key.clone());

        let sig = signing_key
            .sign(&digest)
            .into_bytes()
            .to_vec();

        Ok(sig)
    }
}
