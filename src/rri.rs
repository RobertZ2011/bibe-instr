use num_derive::{ FromPrimitive, ToPrimitive };

use crate::{
    BinOp,
    Register
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum Condition {
	Al,
	Eq,
	Ne,
	Gt,
	Ge,
	Lt,
	Le,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Instruction {
    pub op: BinOp,
    pub cond: Condition,
    pub target: Register,
    pub index: Register,
    pub imm: i16,
}