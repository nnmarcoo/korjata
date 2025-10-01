use crate::core::{marker::Marker, segment::Segment};

pub fn segments(buf: &[u8]) -> Vec<Segment> {
    let mut out = Vec::new();
    let mut i = 0;

    while i < buf.len() {
        if let Some(seg) = parse_segment(buf, i) {
            i += seg.total_size();
            out.push(seg.clone());

            if seg.marker == Marker::SOS {
                break;
            }
        } else {
            break;
        }
    }
    out
}

fn parse_segment(buf: &[u8], offset: usize) -> Option<Segment> {
    if offset + 2 > buf.len() {
        return None;
    }

    let marker_val = u16::from_be_bytes([buf[offset], buf[offset + 1]]);
    let marker = Marker::from_u16(marker_val);

    match marker {
        Marker::SOI | Marker::EOI | Marker::RST(_) => {
            return Some(Segment {
                marker,
                offset,
                data: vec![],
            });
        }
        _ => {}
    }

    if offset + 4 > buf.len() {
        return None;
    }

    let seg_len = u16::from_be_bytes([buf[offset + 2], buf[offset + 3]]) as usize;

    if seg_len < 2 || offset + 2 + seg_len > buf.len() {
        return None;
    }

    Some(Segment {
        marker,
        offset,
        data: buf[offset + 4..offset + 2 + seg_len].to_vec(),
    })
}
