extern crate byteorder;

use std::hash::Hash;
use std::hash::Hasher;

use byteorder::BigEndian;
use byteorder::LittleEndian;
use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;

use std::mem;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Buffer {
    pub data: Vec<u8>,
    pub length: usize,
    pub name: String,
    pub is_little_endian: bool,
}

impl Buffer {
    pub fn from_u8(data: Vec<u8>, name: String, little_endian: bool) -> Buffer {
        let current_length = data.len();
        return Buffer {
            data: data,
            length: current_length,
            name: name,
            is_little_endian: little_endian,
        };
    }

    pub fn describe(&self) -> String {
        return format!("RawBuffer(size={}, elements={:?})", self.length, self.data);
    }

    pub fn get_byte_at(&self, idx: usize) -> Result<u8, String> {
        let element = self.data.get(idx).cloned();
        if element.is_none() {
            return Err(format!("Index {} out of bounds", idx));
        }

        return Ok(element.unwrap());
    }

    pub fn set_byte_at(&mut self, idx: usize, byte: u8) -> Option<String> {
        if idx >= self.length {
            return Some(format!("Index {} out of bounds", idx));
        }

        self.data[idx] = byte;
        return None;
    }

    pub fn get_as_string(&self) -> Result<String, String> {
        let str_repr_result = String::from_utf8(self.data.clone());
        if str_repr_result.is_err() {
            return Err(format!("{}", str_repr_result.unwrap_err()));
        }

        return Ok(str_repr_result.unwrap());
    }

    pub fn get_as_f64(&self) -> Result<f64, String> {
        if self.is_little_endian {
            let f64_repr_result = self.data[..].as_ref().read_f64::<LittleEndian>();
            if f64_repr_result.is_err() {
                return Err(format!("{}", f64_repr_result.unwrap_err()));
            }
            return Ok(f64_repr_result.unwrap());
        } else {
            let f64_repr_result = self.data[..].as_ref().read_f64::<BigEndian>();
            if f64_repr_result.is_err() {
                return Err(format!("{}", f64_repr_result.unwrap_err()));
            }
            return Ok(f64_repr_result.unwrap());
        }
    }

    pub fn get_as_i64(&self) -> Result<i64, String> {
        if self.is_little_endian {
            let i64_repr_result = self.data[..].as_ref().read_i64::<LittleEndian>();
            if i64_repr_result.is_err() {
                return Err(format!("{}", i64_repr_result.unwrap_err()));
            }
            return Ok(i64_repr_result.unwrap());
        } else {
            let i64_repr_result = self.data[..].as_ref().read_i64::<BigEndian>();
            if i64_repr_result.is_err() {
                return Err(format!("{}", i64_repr_result.unwrap_err()));
            }
            return Ok(i64_repr_result.unwrap());
        }
    }

    pub fn from_i64(number: &i64, little_endian: bool) -> Result<Buffer, String> {
        let mut bytes_view = [0u8; mem::size_of::<i64>()];
        if !little_endian {
            let result = bytes_view.as_mut().write_i64::<BigEndian>(*number);
            if result.is_err() {
                return Err(format!("{}", result.unwrap_err()));
            }
        } else {
            let result = bytes_view.as_mut().write_i64::<LittleEndian>(*number);
            if result.is_err() {
                return Err(format!("{}", result.unwrap_err()));
            }
        }

        return Ok(Buffer::from_u8(
            bytes_view.to_vec(),
            "todo".to_string(),
            little_endian,
        ));
    }

    pub fn from_f64(number: &f64, little_endian: bool) -> Result<Buffer, String> {
        let mut bytes_view = [0u8; mem::size_of::<i64>()];
        if !little_endian {
            let result = bytes_view.as_mut().write_f64::<BigEndian>(*number);
            if result.is_err() {
                return Err(format!("{}", result.unwrap_err()));
            }
        } else {
            let result = bytes_view.as_mut().write_f64::<LittleEndian>(*number);
            if result.is_err() {
                return Err(format!("{}", result.unwrap_err()));
            }
        }

        return Ok(Buffer::from_u8(
            bytes_view.to_vec(),
            "todo".to_string(),
            little_endian,
        ));
    }

    pub fn from_string(string: &String) -> Buffer {
        let bytes_repr = string.as_bytes().to_vec();
        return Buffer::from_u8(bytes_repr, "todo".to_string(), false);
    }

    pub fn new_empty(size: usize, is_little_endian: bool) -> Buffer {
        let mut inner = vec![];
        inner.resize(size, 0);

        Self {
            data: inner,
            length: size,
            name: "todo".to_string(),
            is_little_endian,
        }
    }
}

impl Hash for Buffer {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}
