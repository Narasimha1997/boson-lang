extern crate byteorder;

use byteorder::BigEndian;
use byteorder::LittleEndian;
use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;

use crate::compiler::CompiledBytecode;
use crate::compiler::CompiledInstructions;
use crate::types::object::Object;

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
    pub unsafe fn as_type<T: Sized>(buf: &[u8]) -> Option<T> {
        if buf.len() == mem::size_of::<T>() {
            let typed_ref_repr: T = mem::transmute_copy(&buf[0]);
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
            Some(u64_magic) => u64_magic,
            None => 0,
        }
    }
}

#[repr(packed)]
pub struct DataIndexItem {
    pub const_idx: i32,
    pub start: u64,
    pub end: u64,
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
    pub n_locals: u64,
    pub n_params: u64,
    pub code_idx: u64,
    pub const_idx: i32,
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
    pub bin_pool: Vec<u8>,
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
            bin_pool: vec![],
        }
    }

    fn new_data_idx(&mut self, idx: i32, t: TypeCode, data: &[u8]) -> u64 {
        let bin_start = self.bin_pool.len();

        // extend the bin pool with new data:
        self.bin_pool.extend(data);

        let bin_end = self.bin_pool.len();
        let data_item = DataIndexItem {
            const_idx: idx,
            start: bin_start as u64,
            end: bin_end as u64,
            t_code: t,
        };

        self.current_size = bin_end;
        self.data_items.push(data_item);

        return self.data_items.len() as u64;
    }

    fn new_subroutine_idx(
        &mut self,
        const_idx: i32,
        name: String,
        n_p: usize,
        n_l: usize,
        code: &CompiledInstructions,
    ) -> u64 {
        // create a data-index for name:
        let name_data_idx = self.new_data_idx(const_idx, TypeCode::STR, &name.as_bytes());
        let code_idx = self.new_data_idx(const_idx, TypeCode::SUBROUTINE, &code);

        let subroutine = SubroutineIndexItem {
            name_data_idx,
            n_locals: n_l as u64,
            n_params: n_p as u64,
            code_idx,
            const_idx,
        };

        // push to subroutine pool:
        self.subroutine_items.push(subroutine);

        // ad

        return self.subroutine_items.len() as u64;
    }

    pub fn encode_to_binary(&mut self, bytecode: &CompiledBytecode) -> Result<Vec<u8>, String> {
        // prepare the main function subroutine pool:

        // main function:
        self.new_subroutine_idx(-1, "main".to_string(), 0, 0, &bytecode.instructions);

        let mut current_count = 0;
        // now compile the constant pool:
        for object in &bytecode.constant_pool.objects {
            match object.as_ref() {
                Object::Bool(b) => {
                    let b_val = if *b { vec![1u8] } else { vec![0u8] };
                    self.new_data_idx(current_count as i32, TypeCode::BOOL, &b_val);
                }

                _ => {
                    return Err(format!(
                        "Object {} cannot be serialized.",
                        object.get_type()
                    ));
                }
            }
            current_count += 1;
        }

        return Ok(vec![]);
    }
}
