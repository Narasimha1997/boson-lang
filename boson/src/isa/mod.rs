/*
    Defines the instruction types and opcode formatting
    for NP Lang.
*/

use crate::config::BYTE_ENDIAN;

#[repr(u8)]
#[derive(Eq, PartialEq, Debug, Clone)]
pub enum InstructionKind {
    // basic ops
    INoOp,
    IIllegal,

    // arithmetic
    IAdd,
    ISub,
    IMul,
    IDiv,
    IMod,

    // unary arithmetic
    IPreIncr,
    IPreDecr,

    // unary arithmetic
    IPostIncr,
    IPostDecr,

    // bitwise
    IOr,
    IAnd,

    // bitwise unary
    INeg,

    // logical operators
    ILEq,
    ILGt,
    ILGte,
    ILLt,
    ILLTe,
    ILNe,
    ILAnd,
    ILOr,

    // logical unary
    ILNot,

    // Store commandss
    IStoreLocal,
    IStoreGlobal,

    // Load commands
    ILoadLocal,
    ILoadGlobal,

    // Boolean
    ITrue,
    IFlase,

    // Constant
    IConstant,

    // Jump
    IJump,
    INotJump,

    // loops call
    IForEach,
    IWhile,

    // Exception
    IRaise,
    IPushExcHandle,
    IPopExcHandle,

    // Return
    IRet,
    IRetVal,

    // Call
    ICall,
    ICallThread,
    ICallAsync,

    // Call builtin
    ILoadBuiltIn,

    // Closure
    IClosure,

    // Free
    ILoadFree,

    // Data ops:
    IRegArray,
    IRegHash,
    IIndex,

    // NoneType
    INoData,

    // panic instructions will panic the VM, panic
    // instruction will take the top of stack for printing.
    IAssertFail,

    // Iterator:
    IIter,
    IIterNext,
    IEnumNext,

    // Block start and end instructions:
    IBlockStart,
    IBlockEnd,

    // Array and Hash
    IArray,
    IHash,

    // indexing
    IGetIndex,
    ISetIndex,

    // shell call
    IShell,
    IShellRaw,
}


pub type Operands = Vec<usize>;

pub struct InstructionPacker {}

impl InstructionPacker {

    // TODO: Migrate to byteorder crate later.

    fn unpack_16(operand: u16) -> (u8, u8) {
        let x1 = ((operand >> 8) & 0x00FF) as u8;
        let x2 = (operand & 0x00FF) as u8;

        if BYTE_ENDIAN == "big" {
            return (x2, x1);
        }

        return (x1, x2);
    }

    fn pack_u16(x1: u8, x2: u8) -> u16 {
        if BYTE_ENDIAN == "big" {
            return (((x2 as u16) & 0x00FF) << 8) | ((x1 as u16) & 0x00FF);
        } else {
            return (((x1 as u16) & 0x00FF) << 8) | ((x2 as u16) & 0x00FF);
        }
    }

    pub fn encode_instruction(instruction: InstructionKind, operands: &Operands) -> Vec<u8> {
        let operand_sizes = instruction.get_encoding_width();
        let mut statement = Vec::new();

        statement.push(instruction as u8);
        for idx in 0..operands.len() {
            let operand = operands[idx];
            let width = operand_sizes[idx];

            if width == 2 {
                let (x1, x2) = InstructionPacker::unpack_16(operand as u16);
                statement.push(x1);
                statement.push(x2);
            } else {
                statement.push(operand as u8);
            }
        }

        return statement;
    }

    pub fn decode_instruction(
        instruction: &InstructionKind,
        packed_ops: &[u8],
    ) -> (Vec<usize>, usize) {
        let operand_widths = instruction.get_encoding_width();
        let mut unpacked_stmt = vec![];
        let mut offset = 0;

        for width in operand_widths {
            if width == 2 {
                let operand_bytes = &packed_ops[offset..offset + 2];
                let packed_value = InstructionPacker::pack_u16(operand_bytes[0], operand_bytes[1]);
                unpacked_stmt.push(packed_value as usize);
                offset += 2;
            } else {
                unpacked_stmt.push(packed_ops[offset] as usize);
                offset += 1;
            }
        }

        return (unpacked_stmt, offset);
    }
}

impl InstructionKind {
    #[allow(dead_code)]
    pub fn as_string(&self) -> String {
        match self {
            // TODO: Match more and more instructions,

            InstructionKind::IAdd => "IAdd".to_string(),
            InstructionKind::ISub => "ISub".to_string(),
            InstructionKind::IMul => "IMul".to_string(),
            InstructionKind::IDiv => "IDiv".to_string(),
            InstructionKind::IMod => "IMod".to_string(),
            InstructionKind::IAnd => "IAnd".to_string(),
            InstructionKind::IOr => "IOr".to_string(),
            InstructionKind::IConstant => "IConstant".to_string(),
            InstructionKind::IStoreGlobal => "IStoreGlobal".to_string(),
            InstructionKind::IStoreLocal => "IStoreLocal".to_string(),
            InstructionKind::ILoadGlobal => "ILoadGlobal".to_string(),
            InstructionKind::ILoadLocal => "ILoadLocal".to_string(),
            InstructionKind::INeg => "INeg".to_string(),
            InstructionKind::IPostDecr => "IPostDecr".to_string(),
            InstructionKind::IPostIncr => "IPostIncr".to_string(),
            InstructionKind::IPreDecr => "IPreDecr".to_string(),
            InstructionKind::IPreIncr => "IPreIncr".to_string(),
            InstructionKind::IJump => "IJump".to_string(),
            InstructionKind::INotJump => "INotJump".to_string(),
            InstructionKind::INoOp => "INoOp".to_string(),
            InstructionKind::ILLTe => "ILLTe".to_string(),
            InstructionKind::ILLt => "ILLt".to_string(),
            InstructionKind::ILGt => "ILGt".to_string(),
            InstructionKind::ILGte => "ILGte".to_string(),
            InstructionKind::ILEq => "ILEq".to_string(),
            InstructionKind::ILNe => "ILNe".to_string(),
            InstructionKind::ILNot => "ILNot".to_string(),
            InstructionKind::ILOr => "ILOr".to_string(),
            InstructionKind::ILAnd => "ILAnd".to_string(),
            InstructionKind::IAssertFail => "IAssertFail".to_string(),
            InstructionKind::IIter => "IIter".to_string(),
            InstructionKind::IBlockEnd => "IBlockEnd".to_string(),
            InstructionKind::IBlockStart => "IBlockStart".to_string(),
            InstructionKind::IArray => "IArray".to_string(),
            InstructionKind::IHash => "IHash".to_string(),
            InstructionKind::ILoadBuiltIn => "ILoadBuiltIn".to_string(),
            InstructionKind::ICall => "ICall".to_string(),
            InstructionKind::ILoadFree => "ILoadFree".to_string(),
            InstructionKind::IClosure => "IClosure".to_string(),
            InstructionKind::IRet => "IRet".to_string(),
            InstructionKind::IRetVal => "IRetVal".to_string(),
            InstructionKind::IGetIndex => "IGetIndex".to_string(),
            InstructionKind::ISetIndex => "ISetIndex".to_string(),
            InstructionKind::IIterNext => "IIterNext".to_string(),
            InstructionKind::IEnumNext => "IIterEnum".to_string(),
            InstructionKind::IPopExcHandle => "IPopExcHandle".to_string(),
            InstructionKind::IPushExcHandle => "IPushExcHandle".to_string(),
            InstructionKind::IRaise => "IRaise".to_string(),
            InstructionKind::ICallThread => "ICallThread".to_string(),
            InstructionKind::IShell => "IShell".to_string(),
            InstructionKind::IShellRaw => "IShellRaw".to_string(),
            InstructionKind::ICallAsync => "ICallAsync".to_string(),
            _ => "invalid".to_string(),
        }
    }

    #[allow(dead_code)]
    pub fn get_encoding_width(&self) -> Vec<u8> {
        match self {
            InstructionKind::IStoreGlobal
            | InstructionKind::ILoadGlobal
            | InstructionKind::IConstant
            | InstructionKind::IJump
            | InstructionKind::INotJump
            | InstructionKind::IIterNext
            | InstructionKind::IEnumNext 
            | InstructionKind::IHash
            | InstructionKind::IArray
            | InstructionKind::ILoadBuiltIn
            | InstructionKind::ICall
            | InstructionKind::ICallThread
            | InstructionKind::ILoadFree
            | InstructionKind::IPushExcHandle
            | InstructionKind::ICallAsync => vec![2],

            InstructionKind::IAdd
            | InstructionKind::ISub
            | InstructionKind::IMul
            | InstructionKind::IDiv
            | InstructionKind::IAnd
            | InstructionKind::IMod
            | InstructionKind::IOr
            | InstructionKind::INeg
            | InstructionKind::IPreDecr
            | InstructionKind::IPreIncr
            | InstructionKind::IPostIncr
            | InstructionKind::IPostDecr
            | InstructionKind::INoOp
            | InstructionKind::ILGt
            | InstructionKind::ILGte
            | InstructionKind::ILLt
            | InstructionKind::ILLTe
            | InstructionKind::ILNot
            | InstructionKind::ILEq
            | InstructionKind::ILNe
            | InstructionKind::ILOr
            | InstructionKind::IIter
            | InstructionKind::ILAnd
            | InstructionKind::IAssertFail
            | InstructionKind::IBlockEnd
            | InstructionKind::IBlockStart
            | InstructionKind::IRetVal
            | InstructionKind::IRet
            | InstructionKind::IGetIndex
            | InstructionKind::ISetIndex
            | InstructionKind::IRaise
            | InstructionKind::IShellRaw
            | InstructionKind::IShell
            | InstructionKind::IPopExcHandle => vec![],

            InstructionKind::IClosure => vec![2, 2],

            InstructionKind::IStoreLocal
            | InstructionKind::ILoadLocal => vec![1],

            _ => vec![],
        }
    }

    #[allow(dead_code)]
    pub fn disasm_instruction(&self, operands: &Operands) -> String {
        let op_strings: Vec<String> = operands.into_iter().map(|op| format!("{:x}", op)).collect();
        let op_formatted = op_strings.join(", ");
        let opcode = self.as_string();

        return format!("{} {}", opcode, op_formatted);
    }
}
