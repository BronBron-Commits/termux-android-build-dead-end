use anyhow::*;
use std::fs::File;
use std::io::{Read, Write};
use zip::{ZipArchive, ZipWriter, write::FileOptions};

pub struct V1SigAssembler;

impl V1SigAssembler {
    pub fn assemble(
        unsigned_apk: &str,
        manifest_mf: &[u8],
        cert_sf: &[u8],
        cert_rsa: &[u8],
        output_apk: &str,
    ) -> Result<()> {
        let infile = File::open(unsigned_apk)
            .with_context(|| format!("Failed to open unsigned APK: {}", unsigned_apk))?;

        let mut zip_in = ZipArchive::new(infile)
            .context("Invalid ZIP/APK structure")?;

        let outfile = File::create(output_apk)
            .with_context(|| format!("Unable to create output APK: {}", output_apk))?;

        let mut zip_out = ZipWriter::new(outfile);

        let opts = FileOptions::default()
            .compression_method(zip::CompressionMethod::Stored);

        zip_out.start_file("META-INF/MANIFEST.MF", opts)?;
        zip_out.write_all(manifest_mf)?;

        zip_out.start_file("META-INF/CERT.SF", opts)?;
        zip_out.write_all(cert_sf)?;

        zip_out.start_file("META-INF/CERT.RSA", opts)?;
        zip_out.write_all(cert_rsa)?;

        for i in 0..zip_in.len() {
            let mut file = zip_in.by_index(i)?;
            let name = file.name();

            if name.starts_with("META-INF/") {
                continue;
            }

            zip_out.start_file(name, opts)?;
            std::io::copy(&mut file, &mut zip_out)?;
        }

        zip_out.finish()?;
        Ok(())
    }
}
