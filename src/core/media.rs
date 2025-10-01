use std::{fs::File, io::Read, path::PathBuf};

use crate::core::{
    parser::{self, segments},
    segment::Segment,
};

#[derive(Debug)]
pub struct Media {
    buf: Vec<u8>,
    segments: Vec<Segment>,
}

impl Media {
    pub fn from_file(path: &PathBuf) -> Result<Self, String> {
        let mut buf = Vec::new();
        File::open(path)
            .map_err(|e| format!("Failed to open file {:?}: {}", path, e))?
            .read_to_end(&mut buf)
            .map_err(|e| format!("Failed to read file {:?}: {}", path, e))?;

        Ok(Self {
            buf,
            segments: vec![],
        })
    }

    pub fn segments(&self) -> Vec<Segment> {
        segments(&self.buf)
    }
}
