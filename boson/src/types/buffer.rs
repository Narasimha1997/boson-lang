use std::hash::Hash;
use std::hash::Hasher;

#[derive(Clone, Debug, PartialEq)]
pub struct Buffer {
    pub data: Vec<u8>,
    pub length: usize,
    pub name: String,
    pub is_little_endian: bool
}

impl Buffer {

    pub fn new(data: Vec<u8>, name: String) -> Buffer {
        let current_length = data.len();
        return Buffer {
            data: data,
            length: current_length,
            name: name,
            is_little_endian: false,
        };
    }

    pub fn describe(&self) -> String {
        return format!(
            "RawBuffer(size={}, elements={:?})",
            self.length,
            self.data
        );
    }

    pub fn get_byte_at(&self, idx: usize) -> Option<u8> {
        return self.data.get(idx).cloned();
    }

    pub fn set_byte_at(&self, idx: usize, byte: u8) -> Option<String> {
       if idx >= self.length {
           return Some(format!("Index {} out of bounds", idx));
       }

       self.data[idx] = byte;
       return None;
    }

    pub fn get_as_string(&self) -> Result<String, String> {
        let str_repr_result = String::from_utf8(self.data);
        if str_repr_result.is_err() {
            return Err(format!("{}", str_repr_result.unwrap_err()));
        }

        return Ok(str_repr_result.unwrap());
    }
}

impl Hash for Buffer {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}