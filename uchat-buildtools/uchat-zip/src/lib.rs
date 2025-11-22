use anyhow::*;
use std::fs::File;
use std::io::{Read, Write, Seek, SeekFrom};
use zip::{ZipWriter, ZipArchive};
use zip::write::FileOptions;

pub fn read_file_from_zip(path: &str, name: &str) -> Result<Vec<u8>> {
    let file = File::open(path)?;
    let mut zip = ZipArchive::new(file)?;
    let mut entry = zip.by_name(name)?;
    let mut buf = Vec::new();
    entry.read_to_end(&mut buf)?;
    Ok(buf)
}

pub fn replace_file_in_zip(input: &str, output: &str, name: &str, data: &[u8]) -> Result<()> {
    let mut infile = File::open(input)?;
    let mut archive = ZipArchive::new(&mut infile)?;

    let mut out = ZipWriter::new(File::create(output)?);
    let options = FileOptions::default();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let fname = file.name().to_string();

        out.start_file(&fname, options)?;
        if fname == name {
            out.write_all(data)?;
        } else {
            let mut buf = Vec::new();
            file.read_to_end(&mut buf)?;
            out.write_all(&buf)?;
        }
    }

    out.finish()?;
    Ok(())
}
