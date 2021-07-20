use crate::compiler::isa;
use crate::config::BYTE_ENDIAN;

use isa::InstructionKind;

pub type Operands = Vec<usize>;

pub struct InstructionPacker {}

impl InstructionPacker {
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
            // support for only airthmetic and data operations as of now.
            InstructionKind::IAdd => "add".to_string(),
            InstructionKind::ISub => "sub".to_string(),
            InstructionKind::IMul => "mul".to_string(),
            InstructionKind::IDiv => "div".to_string(),
            InstructionKind::IMod => "mod".to_string(),
            InstructionKind::IAnd => "and".to_string(),
            InstructionKind::IOr => "or".to_string(),
            InstructionKind::IConstant => "constant".to_string(),
            InstructionKind::IStoreGlobal => "set_global".to_string(),
            InstructionKind::IStoreLocal => "set_local".to_string(),
            InstructionKind::ILoadGlobal => "get_local".to_string(),
            InstructionKind::ILoadLocal => "get_local".to_string(),
            _ => "invalid".to_string(),
        }
    }

    #[allow(dead_code)]
    pub fn get_encoding_width(&self) -> Vec<u8> {
        match self {
            InstructionKind::IStoreGlobal
            | InstructionKind::ILoadGlobal
            | InstructionKind::IConstant => vec![2],

            InstructionKind::IAdd
            | InstructionKind::ISub
            | InstructionKind::IMul
            | InstructionKind::IDiv
            | InstructionKind::IAnd
            | InstructionKind::IMod
            | InstructionKind::IOr => vec![],

            InstructionKind::IStoreLocal | InstructionKind::ILoadLocal => vec![1],

            _ => vec![],
        }
    }

    #[allow(dead_code)]
    pub fn disasm_instruction(&self, operands: Operands) -> String {
        let op_strings: Vec<String> = operands.into_iter().map(|op| op.to_string()).collect();
        let op_formatted = op_strings.join(", ");
        let opcode = self.as_string();

        return format!("{} {}", opcode, op_formatted);
    }
}
