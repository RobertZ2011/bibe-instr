/* Copyright 2023 Robert Zieba, see LICENSE file for full license. */
use bitfield::bitfield;

pub mod regs;
pub mod registry;

use crate::{
	Encode, Kind, LoadStoreOp, Register
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Instruction {
	pub op: LoadStoreOp,
	pub reg: Register,
	pub imm: u32,
}

bitfield! {
	struct Bitfield(u32);
	impl Debug;
	kind, set_kind : 31, 28;
	op, set_op : 26, 23;
	reg, set_reg : 22, 18;
	imm, set_imm : 17, 0;
}

impl Encode for Instruction {
	fn decode(value: u32) -> Option<Self> {
		let bitfield = Bitfield(value);
		Some(Instruction {
			op: LoadStoreOp::decode(bitfield.op())?,
			reg: Register::new(bitfield.reg() as u8)?,
			imm: bitfield.imm(),
		})
	}

	fn encode(&self) -> u32 {
		let mut bitfield = Bitfield(Kind::Csr.encode());
		bitfield.set_op(self.op.encode());
		bitfield.set_reg(self.reg.as_u8() as u32);
		bitfield.set_imm(self.imm);

		bitfield.0
	}
}