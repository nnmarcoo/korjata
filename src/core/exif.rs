use std::{fs::File, io::Read, path::PathBuf};

pub fn exif(path: &PathBuf) -> Result<i32,> {
    if let Ok(mut file) = File::open(path) {
        let mut buf = vec![];
        file.read_to_end(&mut buf);
    }
}
