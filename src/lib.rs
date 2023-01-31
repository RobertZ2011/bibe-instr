use num_derive::{ FromPrimitive, ToPrimitive };
use num_traits::{ FromPrimitive, ToPrimitive };

pub mod rdrsrs;

#[derive(Clone, Copy, Debug, FromPrimitive, ToPrimitive)]
pub enum Register {
	R0,
	R1,
	R2,
	R3,
	R4,
	R5,
	R6,
	R7,
	R8,
	R9,
	R10,
	R11,
	R12,
	R13,
	R14,
	R15,
	R16,
	R17,
	R18,
	R19,
	R20,
	R21,
	R22,
	R23,
	R24,
	R25,
	R26,
	R27,
	R28,
	R29,
	R30,
	R31,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum Condition {
	Always,
	Eq,
	Ne,
	Gt,
	Gte,
	Lt,
	Lte,
	Res0,
}

#[derive(Clone, Copy, Debug, FromPrimitive, ToPrimitive)]
pub enum Kind {
	RdRsRs,
	RdRsI,
	Memory,
	Res0,
}

#[derive(Clone, Copy, Debug, FromPrimitive, ToPrimitive)]
pub enum RegOp {
	Add,
	Res0,
	Res1,
	Res2,

	Lshift,
	Rshift,
	AlShift,
	ArShift,

	And,
	Or,
	Xor,
	Res3,

	Res4,
	Res5,
	Res6,
	Res7,
	Res8,
	Res9,
	Res10,
	Res11,
	Res12,
	Res13,
	Res14,
	Res15,
}

trait Encode where Self: Sized {
	fn encode(&self) -> u32;
	fn decode(value: u32) -> Self;
}

#[derive(Clone, Copy, Debug)]
pub enum Instruction {
	RdRsRs(Condition, rdrsrs::Instruction)
}

impl Instruction {
	pub fn condition(&self) -> Condition {
		match &self  {
			Instruction::RdRsRs(condition, _) => *condition
		}
	}
}