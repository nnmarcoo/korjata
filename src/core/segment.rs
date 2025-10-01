use crate::core::marker::Marker;

#[derive(Debug, Clone)]
pub struct Segment {
    pub marker: Marker,
    pub offset: usize,
    pub data: Vec<u8>,
}

impl Segment {
    pub fn total_size(&self) -> usize {
        if self.data.is_empty() {
            2
        } else {
            2 + 2 + self.data.len()
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut v = Vec::new();
        v.extend_from_slice(&self.marker.to_u16().to_be_bytes());

        if !self.data.is_empty() {
            v.extend_from_slice(&((self.data.len() + 2) as u16).to_be_bytes());
            v.extend_from_slice(&self.data);
        }
        v
    }
}
