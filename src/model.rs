use num_derive::{ FromPrimitive, ToPrimitive };

use crate::Register;

#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum Operation {
    Read,
    Write,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Instruction {
    op: Operation,
    reg: Register,
    imm: u32,
}