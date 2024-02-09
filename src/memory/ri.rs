/* Copyright 2023 Robert Zieba, see LICENSE file for full license. */
use crate::{
	Encode,
	Kind,
	
	Register,
	util::{
		sign_contract,
		sign_extend,
	},
	LoadStoreOp,
};

use bitfield::bitfield;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Instruction {
	pub op: LoadStoreOp,
	pub rd: Register,
	pub rs: Register,
	pub imm: i16,
}

bitfield! {
	struct Bitfield(u32);
	impl Debug;
	pub kind, set_kind : 31, 28;
	pub op, set_op : 27, 23;
	pub rd, set_rd : 22, 18;
	pub rs, set_rs : 17, 13;
	pub imm, set_imm : 12, 0;
}

impl Encode for Instruction {
	fn decode(value: u32) -> Option<Self> {
		let bitfield = Bitfield(value);
		let op = LoadStoreOp::decode(bitfield.op())?;
		let kind = Kind::decode(value)?;

		if kind != Kind::MemoryRi {
				return None;
		}

		Some(Instruction { 
			op: op,
			rd: Register::new(bitfield.rd() as u8).unwrap(),
			rs: Register::new(bitfield.rs() as u8).unwrap(), 
			imm: sign_extend(bitfield.imm(), 12) as i16,
		})
	}

	fn encode(&self) -> u32 {
		let mut bitfield = Bitfield(Kind::MemoryRi.encode());

		bitfield.set_op(self.op.encode());
		bitfield.set_rd(self.rd.as_u8() as u32);
		bitfield.set_rs(self.rs.as_u8() as u32);
		bitfield.set_imm(sign_contract(self.imm as i32, 14));

		bitfield.0

	}
}