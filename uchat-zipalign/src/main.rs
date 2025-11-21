use anyhow::{Result, anyhow};
use std::fs::File;
use std::io::{Read, Write};
use zip::{ZipArchive, ZipWriter, write::FileOptions, CompressionMethod};

const ALIGNMENT: usize = 4;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        return Err(anyhow!("Usage: uchat-zipalign <input.apk> <output.apk>"));
    }

    let input = &args[1];
    let output = &args[2];

    let infile = File::open(input)?;
    let mut archive = ZipArchive::new(infile)?;

    let outfile = File::create(output)?;
    let mut writer = ZipWriter::new(outfile);

    let mut offset = 0usize;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let name = file.name().to_string();
        let method = file.compression();

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let options = FileOptions::default()
            .compression_method(method)
            .unix_permissions(file.unix_mode().unwrap_or(0o644));

        if method == CompressionMethod::Stored {
            let pad = (ALIGNMENT - (offset % ALIGNMENT)) % ALIGNMENT;
            if pad > 0 {
                writer.write_all(&vec![0u8; pad])?;
                offset += pad;
            }
        }

        writer.start_file(name, options)?;
        writer.write_all(&buffer)?;
        offset += buffer.len();
    }

    writer.finish()?;
    Ok(())
}
