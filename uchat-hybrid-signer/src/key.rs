use anyhow::*;
use ring::signature::RsaKeyPair;

pub fn load_rsa_key(path: &str) -> Result<RsaKeyPair> {
    let pem = std::fs::read(path)?;
    RsaKeyPair::from_pkcs8(&pem)
        .or_else(|_| RsaKeyPair::from_der(&pem))
        .map_err(|_| anyhow!("Invalid RSA key: must be PKCS8 or DER"))
}
