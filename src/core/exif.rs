use std::path::PathBuf;

use nom_exif::{ExifIter, MediaParser, MediaSource, TrackInfo};

pub fn exif(path: &PathBuf) -> String {
    let mut parser = MediaParser::new();

    match MediaSource::file_path(path) {
        Ok(ms) => {
            if ms.has_exif() {
                let mut iter: ExifIter = match parser.parse(ms) {
                    Ok(it) => it,
                    Err(e) => return format!("nom-exif parse error: {:?}", e),
                };

                let mut out = String::new();
                for mut entry in iter {
                    let tag_name = match entry.tag() {
                        Some(t) => format!("{:?}", t),
                        None => format!("0x{:04x}", entry.tag_code()),
                    };
                    let value_str = match entry.get_result() {
                        Ok(v) => format!("{:?}", v),
                        Err(err) => format!("(parse error: {:?})", err),
                    };
                    out.push_str(&format!(
                        "{} (ifd{}): {}\n",
                        tag_name,
                        entry.ifd_index(),
                        value_str
                    ));
                }
                out
            } else if ms.has_track() {
                let info: TrackInfo = match parser.parse(ms) {
                    Ok(t) => t,
                    Err(e) => return format!("nom-exif track parse error: {:?}", e),
                };
                let mut out = String::new();
                for (tag, val) in info.iter() {
                    out.push_str(&format!("{:?}: {:?}\n", tag, val));
                }
                if let Some(gps) = info.get_gps_info() {
                    out.push_str(&format!("GPS: {}\n", gps.format_iso6709()));
                }
                out
            } else {
                "No EXIF/track metadata found (nom-exif)".to_string()
            }
        }
        Err(e) => format!("MediaSource error: {:?}", e),
    }
}
