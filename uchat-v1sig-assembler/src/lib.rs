use anyhow::*;
use uchat_v1sig::v1_hash;

pub fn assemble_v1(data: &[u8]) -> Result<Vec<u8>> {
    let hash = v1_hash(data);
    let mut out = b"V1SIG".to_vec();
    out.extend_from_slice(&hash);
    Ok(out)
}
