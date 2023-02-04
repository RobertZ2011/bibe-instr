use num_derive::{ FromPrimitive, ToPrimitive };

use crate::{ BinOp, Register };

#[derive(Copy, Clone, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum ShiftKind {
    Shl,
    Shr,
    Asl,
    Asr,
    Rol,
    Ror,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Shift {
    pub kind: ShiftKind,
    pub shift: u8,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Instruction {
    pub op: BinOp,
    pub dest: Register,
    pub src1: Register,
    pub src2: Register,
    pub shift: Shift,

}