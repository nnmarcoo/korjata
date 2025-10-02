use chrono::{DateTime, FixedOffset, NaiveDateTime};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct URational {
    pub numerator: u32,
    pub denominator: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IRational {
    pub numerator: i32,
    pub denominator: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Text(String),
    URational(URational),
    IRational(IRational),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    Time(DateTime<FixedOffset>),
    NaiveDateTime(NaiveDateTime),
    Undefined(Vec<u8>),
    URationalArray(Vec<URational>),
    IRationalArray(Vec<IRational>),
    U8Array(Vec<u8>),
    U16Array(Vec<u16>),
    U32Array(Vec<u32>),
}
