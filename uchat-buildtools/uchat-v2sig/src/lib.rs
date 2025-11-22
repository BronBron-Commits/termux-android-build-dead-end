use anyhow::*;
use sha2::{Sha256, Digest};
use byteorder::{LittleEndian, WriteBytesExt};

pub struct V2Signer;

impl V2Signer {
    pub fn sign_v2_block(data: &[u8], private_key: &[u8]) -> Result<Vec<u8>> {
        let h = Sha256::digest(data);
        let mut buf = Vec::new();
        buf.write_u32::<LittleEndian>(0x71777777)?;
        buf.extend_from_slice(&h);
        buf.extend_from_slice(private_key);
        Ok(buf)
    }

    pub fn inject_v2_block(
        unsigned_apk: &str,
        output_apk: &str,
        v2_block: &[u8],
    ) -> Result<()> {
        let mut orig = std::fs::read(unsigned_apk)?;
        orig.extend_from_slice(v2_block);
        std::fs::write(output_apk, orig)?;
        Ok(())
    }
}
