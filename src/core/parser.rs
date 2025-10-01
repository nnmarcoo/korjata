use std::{fs::File, io::Read, path::PathBuf};

use crate::core::{marker::Marker, segment::Segment};

pub fn segments_from_file(path: &PathBuf) -> Result<Vec<Segment>, String> {
    let mut file = File::open(path)
        .map_err(|e| format!("Failed to open file {:?}: {}", path, e))?;

    let mut buf = Vec::new();
    file.read_to_end(&mut buf)
        .map_err(|e| format!("Failed to read file {:?}: {}", path, e))?;

    Ok(get_segments(&buf))
}

pub fn get_segments(buf: &[u8]) -> Vec<Segment> {
    let mut segments = Vec::new();
    let mut i = 0;

    while i < buf.len() {
        if let Some(seg) = Segment::from_buf(buf, i) {
            i += seg.total_size();
            segments.push(seg.clone());

            if seg.marker == Marker::SOS {
                break;
            }
        } else {
            break;
        }
    }

    segments
}