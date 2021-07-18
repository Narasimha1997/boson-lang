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
    IIncr,
    IDecr,

    // bitwise
    IOr,
    IAnd,

    // bitwise unary
    INeg,

    // logical operators
    ILeq,
    ILGt,
    ILGte,
    ILLt,
    ILLTe,
    ILNe,

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
}
