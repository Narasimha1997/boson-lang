extern crate packed_encoder;

use crate::types::{buffer::Buffer, object::Object};
use std::{cell::RefCell, rc::Rc};

use packed_encoder::encoder::{encode_packed, EncodeOrder, EncodeType};

#[inline]
fn to_encodable_type(obj: Rc<Object>, required_type: &str) -> Option<EncodeType> {
    match obj.as_ref() {
        Object::Int(number) => match required_type {
            "uint8" => return Some(EncodeType::Uint8(*number as u8)),
            "uint16" => return Some(EncodeType::Uint16(*number as u16)),
            "uint32" => return Some(EncodeType::Uint32(*number as u32)),
            "uint64" => return Some(EncodeType::Uint64(*number as u64)),
            "uint128" => return Some(EncodeType::Uint128(*number as u128)),
            "int8" => return Some(EncodeType::Int8(*number as i8)),
            "int16" => return Some(EncodeType::Int16(*number as i16)),
            "int32" => return Some(EncodeType::Int32(*number as i32)),
            "int64" => return Some(EncodeType::Int64(*number as i64)),
            "int128" => return Some(EncodeType::Int128(*number as i128)),
            _ => return None,
        },
        Object::Char(ch) => match required_type {
            "uint8" => return Some(EncodeType::Uint8(*ch as u8)),
            "int8" => return Some(EncodeType::Int8(*ch as i8)),
            _ => return None,
        },
        Object::Str(st) => match required_type {
            "string" => return Some(EncodeType::Str(st.to_owned())),
            _ => return None,
        },
        Object::Byte(b) => match required_type {
            "uint8" => return Some(EncodeType::Uint8(*b as u8)),
            "int8" => return Some(EncodeType::Int8(*b as i8)),
            _ => return None,
        },
        Object::ByteBuffer(buffer) => match required_type {
            "bytes" => return Some(EncodeType::Bytes(buffer.borrow().data.clone())),
            _ => return None,
        },
        _ => return None,
    }
}

pub fn encode_boson_types(
    types: &[Rc<Object>],
    data: &[Rc<Object>],
    big_endian: Rc<Object>,
) -> Result<Rc<Object>, String> {
    // native types:
    let mut to_pack_data = vec![];
    for (idx, entry) in types.iter().enumerate() {
        match entry.as_ref() {
            Object::Str(type_def) => {
                if let Some(encodable_type) = to_encodable_type(data[idx].clone(), type_def) {
                    to_pack_data.push(encodable_type);
                } else {
                    return Err(format!(
                        "type {} cannot be encoded as {}",
                        entry.get_type(),
                        type_def
                    ));
                }
            }
            _ => {
                return Err(format!(
                    "types must be a string, but got {}",
                    entry.get_type()
                ))
            }
        }
    }

    let encode_order = if big_endian.is_true() {
        EncodeOrder::Big
    } else {
        EncodeOrder::Little
    };

    if let Ok(bytearray) = encode_packed(&to_pack_data, encode_order) {
        return Ok(Rc::new(Object::ByteBuffer(RefCell::new(Buffer::from_u8(
            bytearray,
            "todo".to_owned(),
            !big_endian.is_true(),
        )))));
    }

    Err(format!("encoding failed for given types"))
}
