use anyhow::{Result, anyhow};
use uchat_v1sig::V1SignatureGenerator;
use uchat_v1sig_assembler::V1SignatureAssembler;
use uchat_v2sig::V2SignatureBlock;

pub enum SignVersion {
    V1,
    V2,
}

pub struct UChatSigner;

impl UChatSigner {
    pub fn sign(
        version: SignVersion,
        key_pem: &[u8],
        input_apk: &[u8],
    ) -> Result<Vec<u8>> {
        match version {
            SignVersion::V1 => Self::sign_v1(key_pem, input_apk),
            SignVersion::V2 => Self::sign_v2(key_pem, input_apk),
        }
    }

    fn sign_v1(key_pem: &[u8], apk_data: &[u8]) -> Result<Vec<u8>> {
        let dg = V1SignatureGenerator::new(key_pem)?;
        let (manifest, cert_sf, cert_rsa) = dg.generate_all(apk_data)?;

        let mut asm = V1SignatureAssembler::new();
        asm.attach_apk(apk_data)?;
        asm.inject_v1(&manifest, &cert_sf, &cert_rsa)?;
        asm.finalize()
    }

    fn sign_v2(key_pem: &[u8], apk_data: &[u8]) -> Result<Vec<u8>> {
        let block = V2SignatureBlock::sign_with_v2block(key_pem, apk_data)?;
        let mut out = Vec::new();
        out.extend_from_slice(apk_data);
        out.extend_from_slice(&block);
        Ok(out)
    }

    pub fn autodetect_and_sign(
        key_pem: &[u8],
        input_apk: &[u8],
    ) -> Result<Vec<u8>> {
        if input_apk.starts_with(b"PK") {
            Self::sign(SignVersion::V1, key_pem, input_apk)
        } else {
            Err(anyhow!("Unknown APK format"))
        }
    }
}
