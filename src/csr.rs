/* Copyright 2023 Robert Zieba, see LICENSE file for full license. */
use num_derive::{ FromPrimitive, ToPrimitive };
use num_traits::{ FromPrimitive, ToPrimitive };
use bitfield::bitfield;

use crate::{
	Encode,
	Kind,
	Register,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum Operation {
	Read,
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
	Res10,
	Res11,
	Res12,
	Res13,
	Res14,

	Write,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Instruction {
	pub op: Operation,
	pub reg: Register,
	pub imm: u32,
}

bitfield! {
	struct Bitfield(u32);
	impl Debug;
	kind, set_kind : 31, 28;
	op, set_op : 27, 23;
	reg, set_reg : 22, 18;
	imm, set_imm : 17, 0;
}

impl Encode for Instruction {
	fn decode(value: u32) -> Option<Self> {
		let bitfield = Bitfield(value);
		Some(Instruction {
			op: Operation::from_u32(bitfield.op())?,
			reg: Register::new(bitfield.reg() as u8)?,
			imm: bitfield.imm(),
		})
	}

	fn encode(&self) -> u32 {
		let mut bitfield = Bitfield(0);
		bitfield.set_kind(Kind::Csr.to_u32().unwrap());
		bitfield.set_op(self.op.to_u32().unwrap());
		bitfield.set_reg(self.reg.as_u8() as u32);
		bitfield.set_imm(self.imm);

		bitfield.0
	}
}