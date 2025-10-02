use crate::core::{attribute::Attribute, value::Value};

#[derive(Debug, Clone)]
pub struct Entry {
    pub attribute: &'static Attribute,
    pub offset: usize,
    pub raw_count: u32,
    pub value: Value,
}