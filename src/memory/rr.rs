/* Copyright 2023 Robert Zieba, see LICENSE file for full license. */
use crate::{
	Encode,
	Kind,
	LoadStoreOp,
	Shift,
	ShiftKind,
	Register,
};

use num_traits::FromPrimitive;

use bitfield::bitfield;

bitfield! {
	struct Bitfield(u32);
	impl Debug;
	pub kind, set_kind : 31, 28;
	pub op, set_op : 26, 23;
	pub rd, set_rd : 22, 18;
	pub rs, set_rs : 17, 13;
	pub rq, set_rq : 12, 8;
	pub shift_kind, set_shift_kind : 7, 5;
	pub shift, set_shift : 4, 0;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Instruction {
	pub op: LoadStoreOp,
	pub rd: Register,
	pub rs: Register,
	pub rq: Register,
	pub shift: Shift,
}

impl Encode for Instruction {
	fn decode(value: u32) -> Option<Self> {
		let bitfield = Bitfield(value);
		let op = LoadStoreOp::decode(bitfield.op())?;
		let kind = Kind::decode(value)?;

		if kind != Kind::MemoryRr {
				return None;
		}

		Some(Instruction {
			op: op,
			rd: Register::new(bitfield.rd() as u8).unwrap(),
			rs: Register::new(bitfield.rs() as u8).unwrap(),
			rq: Register::new(bitfield.rq() as u8).unwrap(),
			shift: Shift {
				kind: ShiftKind::from_u8(bitfield.shift_kind() as u8)?,
				shift: bitfield.shift() as u8,
			}
		})
	}

	fn encode(&self) -> u32 {
		let mut bitfield = Bitfield(Kind::MemoryRr.encode());

		bitfield.set_op(self.op.encode());
		bitfield.set_rd(self.rd.as_u8() as u32);
		bitfield.set_rs(self.rs.as_u8() as u32);
		bitfield.set_rq(self.rq.as_u8() as u32);
		bitfield.set_shift_kind(self.shift.kind.encode());
		bitfield.set_shift(self.shift.shift as u32);

		bitfield.0

	}
}