use num_enum::TryFromPrimitive;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Count {
    Fixed(u32),
    Variable,
}

#[derive(Debug, Clone, Copy)]
pub struct Attribute {
    pub tag_name: &'static str,
    pub field_name: &'static str,
    pub fid: u16,
    pub exif_type: ExifType,
    pub count: Count,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u16)]
pub enum ExifType {
    Byte = 1,
    Ascii = 2,
    Short = 3,
    Long = 4,
    Rational = 5,
    SByte = 6,
    Undefined = 7,
    SShort = 8,
    SLong = 9,
    SRational = 10,
    Float = 11,
    Double = 12,
}

impl ExifType {
    pub fn size(&self) -> usize {
        match self {
            ExifType::Byte | ExifType::Ascii | ExifType::SByte | ExifType::Undefined => 1,
            ExifType::Short | ExifType::SShort => 2,
            ExifType::Long | ExifType::SLong | ExifType::Float => 4,
            ExifType::Rational | ExifType::SRational | ExifType::Double => 8,
        }
    }
}
