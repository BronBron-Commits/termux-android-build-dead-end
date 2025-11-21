mod fakesha1;

use rsa::{pkcs1::EncodeRsaPrivateKey, pkcs1v15::SigningKey, RsaPrivateKey};
use rand::rngs::OsRng;
use fakesha1::FakeSha1;

fn main() {
    let mut rng = OsRng;
    let key = RsaPrivateKey::new(&mut rng, 2048).expect("key gen failed");
    let _der = key.to_pkcs1_der().expect("encode failed").as_bytes().to_vec();

    let signing_key = SigningKey::<FakeSha1>::new(key);
    println!("Signing key ready.");
}
