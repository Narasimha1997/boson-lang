/*
    Defines the instruction types and opcode formatting
    for NP Lang.
*/

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

    // Return
    IRet,
    IRetVal,

    // Call
    ICall,

    // Closure
    IClosure,

    // Free
    IGetFree,

    // Data ops:
    IRegArray,
    IRegHash,
    IIndex,

    // NoneType
    INoData,

    // panic instructions will panic the VM, panic
    // instruction will take the top of stack for printing.
    IVMPanic,

    // Iterator:
    IIter,

    // Block start and end instructions:
    IBlockStart,
    IBlockEnd,
}
