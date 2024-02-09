/* Copyright 2023 Robert Zieba, see LICENSE file for full license. */
use bitfield::bitfield;
use log::debug;
use num_derive::{ FromPrimitive, ToPrimitive };
use num_traits::{ FromPrimitive, ToPrimitive };

use crate::{
	Encode,
	BinOp,
	Kind,
	Register,
	util::{
		sign_contract,
		sign_extend,
	},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum Condition {
	Always,
	Overflow,
	Carry,
	Zero,
	Negative,
	NotZero,
	NotNegative,
	GreaterThan,
}

impl Encode for Condition {
	fn decode(value: u32) -> Option<Self> {
		Condition::from_u32(value)
	}

	fn encode(&self) -> u32 {
		self.to_u32().unwrap()
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Instruction {
	pub op: BinOp,
	pub cond: Condition,
	pub dest: Register,
	pub src: Register,
	pub imm: i16,
}

bitfield! {
	struct Bitfield(u32);
	impl Debug;
	pub kind, set_kind : 31, 30;
	pub op, set_op : 29, 25;
	pub rd, set_rd : 24, 20;
	pub rs, set_rs : 19, 15;
	pub cond, set_cond : 14, 12;
	pub imm, set_imm : 11, 0;
}

impl Encode for Instruction {
	fn decode(value: u32) -> Option<Instruction> {
		let bitfield = Bitfield(value);
		let kind = Kind::decode(value)?;
	
		if kind != Kind::Rri {
			debug!("Not an RRI instruction, got {:?}", kind);
			return None;
		}

		Some(Instruction {
			op: BinOp::from_u32(bitfield.op())?,
			cond: Condition::from_u32(bitfield.cond())?,
			dest: Register::new(bitfield.rd() as u8).unwrap(),
			src: Register::new(bitfield.rs() as u8).unwrap(),
			imm: sign_extend(bitfield.imm(), 12) as i16,
		})
	}

	fn encode(&self) -> u32 {
		let mut bitfield = Bitfield(Kind::Rri.encode());
		bitfield.set_op(self.op.encode());
		bitfield.set_rd(self.dest.encode());
		bitfield.set_rs(self.src.encode());
		bitfield.set_cond(self.cond.encode());
		bitfield.set_imm(sign_contract(self.imm as i32, 12));
		bitfield.0
	}
}