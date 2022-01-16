fn main() {
    let _ = decompress_a_tarball();
}

fn decompress_a_tarball() -> Result<(), std::io::Error> {
    use std::fs::File;
    use flate2::read::GzDecoder;
    use tar::Archive;

    let path = "archive.tar.gz";

    let tar_gz = File::open(path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack("./dst")?;

    Ok(())
}