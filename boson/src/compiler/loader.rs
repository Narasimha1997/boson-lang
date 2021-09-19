use crate::isa::InstructionKind;
use crate::isa::InstructionPacker;

use std::slice;
use std::mem;

#[allow(dead_code)]
pub enum TypeCode {
    NONE,
    INT,
    STR,
    FLOAT,
    BOOL,
    SUBROUTINE,
}

pub struct BytecodeSaver {}
pub struct BytecodeLoader {}


pub struct ByteOps {}

// takes a sized struct and returns the in-memory byte representation
// zero-copy, see: https://stackoverflow.com/questions/28127165/how-to-convert-struct-to-u8
#[allow(dead_code)]
unsafe fn to_bytes<S:Sized>(s: &S) -> &[u8] {
    let byte_slice_repr = slice::from_raw_parts(
        (s as *const S) as *const u8,
        mem::size_of::<S>()
    );

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
pub struct DataIndex {
   pub num_entries: u32,
   pub end_index: u64,
}


