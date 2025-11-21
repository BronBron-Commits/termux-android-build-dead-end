use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use zip::{ZipArchive, ZipWriter};
use zip::write::FileOptions;
use tempfile::tempdir;

fn main() -> std::io::Result<()> {
    let input_apk = "app-unsigned.apk";
    let output_apk = "app-signed.apk";

    let manifest_content = "\
Manifest-Version: 1.0
Created-By: uchat-apksigner
";

    let cert_sf_content = "\
Signature-Version: 1.0
Created-By: uchat-apksigner

Name: res/layout/activity_main.xml
SHA1-Digest: dummyhash1234567890==
";

    let cert_rsa_content = b"DUMMY SIGNATURE BLOCK PLACEHOLDER";

    let input_file = File::open(input_apk)?;
    let mut zip_reader = ZipArchive::new(input_file)?;
    let temp_dir = tempdir()?;
    let temp_path = temp_dir.path().join("signed.apk");
    let output_file = File::create(&temp_path)?;
    let mut zip_writer = ZipWriter::new(output_file);

    for i in 0..zip_reader.len() {
        let mut file = zip_reader.by_index(i)?;
        let name = file.name().to_string();

        if name.starts_with("META-INF/") {
            continue; // Strip all old signature files
        }

        let options = FileOptions::default()
            .compression_method(file.compression())
            .last_modified_time(file.last_modified());

        zip_writer.start_file(name.clone(), options)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        zip_writer.write_all(&buffer)?;
    }

    // Add new META-INF files
    zip_writer.start_file("META-INF/MANIFEST.MF", FileOptions::default())?;
    zip_writer.write_all(manifest_content.as_bytes())?;

    zip_writer.start_file("META-INF/CERT.SF", FileOptions::default())?;
    zip_writer.write_all(cert_sf_content.as_bytes())?;

    zip_writer.start_file("META-INF/CERT.RSA", FileOptions::default())?;
    zip_writer.write_all(cert_rsa_content)?;

    zip_writer.finish()?;
    fs::copy(&temp_path, output_apk)?;
    println!("Signed APK written to {}", output_apk);

    Ok(())
}
