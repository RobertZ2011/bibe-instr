/* Copyright 2023 Robert Zieba, see LICENSE file for full license. */
use num_derive::{ FromPrimitive, ToPrimitive };
use num_traits::{ FromPrimitive, ToPrimitive };
use bitfield::bitfield;

use crate::{
	Encode,
	Kind,
	Register,
	util:: {
		sign_contract,
		sign_extend,
	},
	Width,
};

pub mod rr;
pub mod ri;

#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum Operation {
	LoadBRr,
	StoreBRr,
	LoadSRr,
	StoreSRr,
	LoadWRr,
	StoreWRr,
	Res0,
	Res1,

	Res2,
	Res3,
	Res4,
	Res5,
	Res6,
	Res7,
	Res8,
	Res9,

	LoadBRi,
	StoreBRi,
	LoadSRi,
	StoreSRi,
	LoadWRi,
	StoreWRi,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OpType {
	Load,
	Store,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AddressMode {
	Rr,
	Ri,
}

impl Operation {
	pub fn from_parts(mode: AddressMode, kind: OpType, width: Width) -> Option<Self> {
		match (mode, kind, width) {
			(AddressMode::Rr, OpType::Load, Width::Byte) => Some(Operation::LoadBRr),
			(AddressMode::Rr, OpType::Store, Width::Byte) => Some(Operation::StoreBRr),

			(AddressMode::Rr, OpType::Load, Width::Short) => Some(Operation::LoadSRr),
			(AddressMode::Rr, OpType::Store, Width::Short) => Some(Operation::StoreSRr),

			(AddressMode::Rr, OpType::Load, Width::Word) => Some(Operation::LoadWRr),
			(AddressMode::Rr, OpType::Store, Width::Word) => Some(Operation::StoreWRr),

			(AddressMode::Ri, OpType::Load, Width::Byte) => Some(Operation::LoadBRi),
			(AddressMode::Ri, OpType::Store, Width::Byte) => Some(Operation::StoreBRi),

			(AddressMode::Ri, OpType::Load, Width::Short) => Some(Operation::LoadSRi),
			(AddressMode::Ri, OpType::Store, Width::Short) => Some(Operation::StoreSRi),

			(AddressMode::Ri, OpType::Load, Width::Word) => Some(Operation::LoadWRi),
			(AddressMode::Ri, OpType::Store, Width::Word) => Some(Operation::StoreWRi),

			_ => None,
		}
	}

	pub fn parts(&self) -> Option<(AddressMode, OpType, Width)> {
		match self {
			Operation::LoadBRr => Some((AddressMode::Rr, OpType::Load, Width::Byte)),
			Operation::StoreBRr => Some((AddressMode::Rr, OpType::Store, Width::Byte)),

			Operation::LoadSRr => Some((AddressMode::Rr, OpType::Load, Width::Short)),
			Operation::StoreSRr => Some((AddressMode::Rr, OpType::Store, Width::Short)),

			Operation::LoadWRr => Some((AddressMode::Rr, OpType::Load, Width::Word)),
			Operation::StoreWRr => Some((AddressMode::Rr, OpType::Store, Width::Word)),


			Operation::LoadBRi => Some((AddressMode::Ri, OpType::Load, Width::Byte)),
			Operation::StoreBRi => Some((AddressMode::Ri, OpType::Store, Width::Byte)),

			Operation::LoadSRi => Some((AddressMode::Ri, OpType::Load, Width::Short)),
			Operation::StoreSRi => Some((AddressMode::Ri, OpType::Store, Width::Short)),

			Operation::LoadWRi => Some((AddressMode::Ri, OpType::Load, Width::Word)),
			Operation::StoreWRi => Some((AddressMode::Ri, OpType::Store, Width::Word)),

			_ => None,
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Instruction {
	Rr(rr::Instruction),
	Ri(ri::Instruction),
}

bitfield! {
	struct Bitfield(u32);
	impl Debug;
	pub kind, set_kind : 31, 28;
	pub op, set_op : 27, 23;
}

impl Encode for Instruction {
	fn decode(value: u32) -> Option<Instruction> {
		let bitfield = Bitfield(value);

		let kind = Kind::from_u32(bitfield.kind())?;
		if kind != Kind::Memory {
			return None;
		}

		let (mode, _, _) = Operation::from_u32(bitfield.op())?.parts()?;
		match mode {
			AddressMode::Rr => Some(Instruction::Rr(rr::Instruction::decode(value)?)),
			AddressMode::Ri=> Some(Instruction::Ri(ri::Instruction::decode(value)?)),
		}
	}

	fn encode(&self) -> u32 {
		match self {
			Instruction::Rr(i) => i.encode(),
			Instruction::Ri(i) => i.encode(),
		}
	}
}