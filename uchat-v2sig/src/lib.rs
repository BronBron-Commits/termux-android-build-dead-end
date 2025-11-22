use anyhow::*;
use ring::signature::{KeyPair, Ed25519KeyPair};

pub fn sign_v2(data: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    let kp = Ed25519KeyPair::from_seed_unchecked(key)
        .map_err(|_| anyhow!("Invalid Ed25519 seed"))?;
    Ok(kp.sign(data).as_ref().to_vec())
}
