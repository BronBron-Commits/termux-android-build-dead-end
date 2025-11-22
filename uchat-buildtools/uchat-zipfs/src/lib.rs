use anyhow::*;
use std::fs::File;
use std::io::{Read, Write};
use zip::{ZipArchive, ZipWriter};
use zip::write::FileOptions;

pub fn extract_file(apk_path: &str, file_name: &str) -> Result<Vec<u8>> {
    let mut file = File::open(apk_path)
        .with_context(|| format!("Failed to open APK: {}", apk_path))?;

    let mut archive = ZipArchive::new(&mut file)
        .context("Failed to parse APK as ZIP")?;

    let mut target = archive
        .by_name(file_name)
        .with_context(|| format!("File not found in APK: {}", file_name))?;

    let mut buf = Vec::new();
    target.read_to_end(&mut buf)?;
    Ok(buf)
}

pub fn create_signed_apk(
    unsigned_apk: &str,
    out_apk: &str,
    manifest: &[u8],
    cert_sf: &[u8],
    cert_rsa: &[u8],
) -> Result<()> {
    let mut infile = File::open(unsigned_apk)
        .with_context(|| format!("Failed to open unsigned APK: {}", unsigned_apk))?;

    let mut zin = ZipArchive::new(&mut infile)?;

    let outfile = File::create(out_apk)
        .with_context(|| format!("Failed to create output APK: {}", out_apk))?;

    let mut zout = ZipWriter::new(outfile);

    let opts = FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored);

    // Write signature files first
    zout.start_file("META-INF/MANIFEST.MF", opts)?;
    zout.write_all(manifest)?;

    zout.start_file("META-INF/CERT.SF", opts)?;
    zout.write_all(cert_sf)?;

    zout.start_file("META-INF/CERT.RSA", opts)?;
    zout.write_all(cert_rsa)?;

    // Copy all other files except META-INF/*
    for i in 0..zin.len() {
        let mut f = zin.by_index(i)?;
        let name = f.name().to_string();

        if name.starts_with("META-INF/") {
            continue;
        }

        zout.start_file(name, opts)?;
        std::io::copy(&mut f, &mut zout)?;
    }

    zout.finish()?;
    Ok(())
}
