use std::{fs::File, io::Read, path::PathBuf};

#[derive(Debug)]
pub enum App1Block<'a> {
    Exif(&'a [u8]),
    Xmp(&'a [u8]),
    Other(&'a [u8]),
}

pub fn exif(path: &PathBuf) -> Result<Vec<usize>, String> {
    if let Ok(mut file) = File::open(path) {
        let mut buf = vec![];
        let _ = file.read_to_end(&mut buf);

        return Ok(find_app1_segments(&buf));
    }

    Ok(vec![])
}

pub fn find_app1_segments(buf: &[u8]) -> Vec<usize> {
    let mut blocks = Vec::new();
    let mut i = 2;

    while i + 4 <= buf.len() {
        if buf[i] != 0xFF {
            i += 1;
            continue;
        }

        let marker = buf[i + 1];

        match marker {
            0xDA => break,
            0xE1 => {
                let seg_len = u16::from_be_bytes([buf[i + 2], buf[i + 3]]) as usize;
                if i + 2 + seg_len > buf.len() {
                    break;
                }
                blocks.push(i);
                i += 2 + seg_len;
            }
            _ => i += 2,
        }
    }
    blocks
}
