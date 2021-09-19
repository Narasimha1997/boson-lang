extern crate byteorder;

use byteorder::BigEndian;
use byteorder::LittleEndian;
use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;

use crate::compiler::CompiledBytecode;

use std::mem;
use std::slice;

const USE_BIG_ENDIAN_REPR: bool = false;
const MAGIC: &str = "000BOSON";

#[allow(dead_code)]
#[repr(u8)]
pub enum TypeCode {
    NONE,
    CHAR,
    BYTE,
    INT,
    STR,
    FLOAT,
    BOOL,
    SUBROUTINE,
    CODE,
}

pub struct BytecodeSaver {}
pub struct BytecodeLoader {}

pub struct ByteOps {}

impl ByteOps {
    // takes a sized struct and returns the in-memory byte representation
    // zero-copy, see: https://stackoverflow.com/questions/28127165/how-to-convert-struct-to-u8
    pub unsafe fn as_bytes<S: Sized>(s: &S) -> &[u8] {
        let byte_slice_repr =
            slice::from_raw_parts((s as *const S) as *const u8, mem::size_of::<S>());
        return byte_slice_repr;
    }

    // returns the typed representation of a slice of bytes
    // zero-copy, this just returns the typed reference, does not copy any data.
    pub unsafe fn as_type<T: Sized>(buf: &[u8]) -> Option<&T> {
        if buf.len() == mem::size_of::<T>() {
            let typed_ref_repr: &T = mem::transmute(&buf[0]);
            return Some(typed_ref_repr);
        } else {
            return None;
        }
    }

    pub fn repr_boson_float(f: &f64) -> Option<Vec<u8>> {
        let mut bytes = [0u8; mem::size_of::<f64>()];
        if USE_BIG_ENDIAN_REPR {
            let result = bytes.as_mut().write_f64::<BigEndian>(*f);
            if result.is_err() {
                return None;
            }
        } else {
            let result = bytes.as_mut().write_f64::<LittleEndian>(*f);
            if result.is_err() {
                return None;
            }
        }

        return Some(bytes.to_vec());
    }

    pub fn repr_boson_int(i: &i64) -> Option<Vec<u8>> {
        let mut bytes = [0u8; mem::size_of::<f64>()];
        if USE_BIG_ENDIAN_REPR {
            let result = bytes.as_mut().write_i64::<BigEndian>(*i);
            if result.is_err() {
                return None;
            }
        } else {
            let result = bytes.as_mut().write_i64::<LittleEndian>(*i);
            if result.is_err() {
                return None;
            }
        }

        return Some(bytes.to_vec());
    }

    pub fn generate_magic() -> u64 {
        let result = unsafe { ByteOps::as_type::<u64>(MAGIC.as_bytes()) };

        match result {
            Some(u64_magic) => *u64_magic,
            None => 0,
        }
    }
}

#[repr(packed)]
pub struct DataIndexItem {
    pub const_idx: u32,
    pub start: u32,
    pub end: u32,
    pub t_code: TypeCode,
}

#[repr(packed)]
pub struct Header {
    pub magic: u64,
    pub num_data: u32,
    pub num_sub: u32,
    pub data_end_idx: u64,
    pub sub_end_idx: u64,
}

#[repr(packed)]
pub struct SubroutineIndexItem {
    pub name_data_idx: u64,
    pub n_locals_data_idx: u64,
    pub n_params_data_idx: u64,
    pub code_idx: u64,
}

// organization of bytecode file:

/*
    | header = contains info about subroutine-index and data index |
    | subroutine-index = mapping to subroutine in bytes area |
    | data-index = contains mapping of data in bytes area|
    | bytes-area |
*/

pub struct BytecodeWriter {
    pub current_size: usize,
    pub data_items: Vec<DataIndexItem>,
    pub subroutine_items: Vec<SubroutineIndexItem>,
    pub header: Header,
}

impl BytecodeWriter {
    pub fn new() -> BytecodeWriter {
        BytecodeWriter {
            current_size: 0,
            data_items: vec![],
            subroutine_items: vec![],
            header: Header {
                magic: ByteOps::generate_magic(),
                num_data: 0,
                num_sub: 0,
                data_end_idx: 0,
                sub_end_idx: 0,
            },
        }
    }

    pub fn encode_to_binary(_bytecode: &CompiledBytecode) {}
}
