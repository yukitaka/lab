fn main() {
    let _ = decompress_a_tarball();
    let _ = compress_a_directory_into_tarball();
}

fn decompress_a_tarball() -> Result<(), std::io::Error> {
    use flate2::read::GzDecoder;
    use std::fs::File;
    use tar::Archive;

    let path = "archive.tar.gz";

    let tar_gz = File::open(path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack("./dst")?;

    Ok(())
}

fn compress_a_directory_into_tarball() -> Result<(), std::io::Error> {
    use flate2::write::GzEncoder;
    use flate2::Compression;
    use std::fs::File;

    let tar_gz = File::create("archive.tar.gz")?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    tar.append_dir_all("backup/logs", "/var/log")?;
    Ok(())
}
