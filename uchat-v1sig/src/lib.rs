pub fn v1_hash(data: &[u8]) -> [u8; 32] {
    use sha2::{Sha256, Digest};
    let mut h = Sha256::new();
    h.update(data);
    h.finalize().into()
}
