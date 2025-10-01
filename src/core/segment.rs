use crate::core::marker::Marker;

#[derive(Debug, Clone)]
pub struct Segment {
    pub marker: Marker,
    pub offset: usize,
    pub length: usize,
    pub data: Vec<u8>,
}

impl Segment {
    pub fn from_buf(buf: &[u8], offset: usize) -> Option<Self> {
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
                    length: 0,
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
            length: seg_len - 2,
            data: buf[offset + 4..offset + 2 + seg_len].to_vec(),
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut v = Vec::new();
        v.extend_from_slice(&self.marker.to_u16().to_be_bytes());

        if self.length > 0 || !self.data.is_empty() {
            v.extend_from_slice(&((self.data.len() + 2) as u16).to_be_bytes());
            v.extend_from_slice(&self.data);
        }
        v
    }

    pub fn total_size(&self) -> usize {
        if self.length == 0 {
            2
        } else {
            2 + 2 + self.length
        }
    }
}
