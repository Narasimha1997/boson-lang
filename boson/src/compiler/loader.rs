use crate::isa::InstructionKind;
use crate::isa::InstructionPacker;

use std::mem;
use std::slice;

#[allow(dead_code)]
#[repr(u8)]
pub enum TypeCode {
    NONE,
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

// takes a sized struct and returns the in-memory byte representation
// zero-copy, see: https://stackoverflow.com/questions/28127165/how-to-convert-struct-to-u8
#[allow(dead_code)]
unsafe fn to_bytes<S: Sized>(s: &S) -> &[u8] {
    let byte_slice_repr = slice::from_raw_parts((s as *const S) as *const u8, mem::size_of::<S>());

    return byte_slice_repr;
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
