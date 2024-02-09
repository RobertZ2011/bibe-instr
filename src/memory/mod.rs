/* Copyright 2023 Robert Zieba, see LICENSE file for full license. */
use bitfield::bitfield;

use crate::{
	Encode,
	Kind,
};

pub mod rr;
pub mod ri;

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
		let kind = Kind::decode(value)?;

		match kind {
			Kind::MemoryRr => Some(Instruction::Rr(rr::Instruction::decode(value)?)),
			Kind::MemoryRi => Some(Instruction::Ri(ri::Instruction::decode(value)?)),
			_ => None,
		}
	}

	fn encode(&self) -> u32 {
		match self {
			Instruction::Rr(i) => i.encode(),
			Instruction::Ri(i) => i.encode(),
		}
	}
}