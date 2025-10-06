use std::{fs::File, io::Read, path::PathBuf};

use crate::core::{parse::segments, segment::Segment};

#[derive(Debug, Default)]
pub struct Media {
    pub path: PathBuf,
    buf: Vec<u8>,
}

impl Media {
    pub fn from_file(path: PathBuf) -> Result<Self, String> {
        let mut buf = Vec::new();
        File::open(&path)
            .map_err(|e| format!("Failed to open file {:?}: {}", path, e))?
            .read_to_end(&mut buf)
            .map_err(|e| format!("Failed to read file {:?}: {}", path, e))?;
        Ok(Self { path, buf })
    }

    pub fn segments(&self) -> Vec<Segment> {
        segments(&self.buf)
    }

    pub fn path_string(&self) -> String {
        self.path.to_str().unwrap_or("-").to_string()
    }
}
