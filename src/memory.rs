use num_derive::{ FromPrimitive, ToPrimitive };

use crate::Register;

#[derive(Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum Operation {
	LoadB,
	StoreB,
	LoadS,
	StoreS,
	LoadW,
	StoreW,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum OpType {
	Load,
	Store,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum Width {
	W1,
	W2,
	W4,
}

impl Operation {
	pub fn parts(&self) -> (OpType, Width) {
		match self {
			Operation::LoadB => (OpType::Load, Width::W1),
			Operation::StoreB => (OpType::Store, Width::W1),
			Operation::LoadS => (OpType::Load, Width::W2),
			Operation::StoreS => (OpType::Store, Width::W2),
			Operation::LoadW => (OpType::Load, Width::W4),
			Operation::StoreW => (OpType::Store, Width::W4),
		}
	}
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Instruction {
	op: OpType,
	dest: Register,
	src: Register,
	shift: i8,
	immediate: i32,
}