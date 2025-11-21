use std::fs::{File};
use std::io::{Write};
use zip::write::FileOptions;
use zip::ZipWriter;
use rcgen::{Certificate, CertificateParams, PKCS_RSA_SHA256};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Generate a basic self-signed certificate
    let mut params = CertificateParams::new(vec!["uchat".into()]);
    params.alg = &PKCS_RSA_SHA256;
    let cert = Certificate::from_params(params)?;

    let cert_der = cert.serialize_der()?;
    let key_der = cert.serialize_private_key_der();

    // Create META-INF/CERT.RSA as zip
    let out_file = File::create("CERT.RSA")?;
    let mut zip = ZipWriter::new(out_file);

    let options = FileOptions::default().compression_method(zip::CompressionMethod::Stored);

    zip.start_file("CERT.DER", options)?;
    zip.write_all(&cert_der)?;

    zip.start_file("PRIVATEKEY.DER", options)?;
    zip.write_all(&key_der)?;

    zip.finish()?;
    Ok(())
}
