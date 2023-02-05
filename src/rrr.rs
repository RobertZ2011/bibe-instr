use bitfield::bitfield;
use num_derive::{ FromPrimitive, ToPrimitive };
use num_traits::{ FromPrimitive, ToPrimitive };

use crate::{
	BinOp,
	Encode,
	Kind,
	Register,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum ShiftKind {
	Shl,
	Shr,
	Asl,
	Asr,
	Rol,
	Ror,
}

impl Encode for ShiftKind {
	fn decode(value: u32) -> Option<ShiftKind> {
		ShiftKind::from_u32(value)
	}

	fn encode(&self) -> u32 {
		self.to_u32().unwrap()
	}
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Shift {
	pub kind: ShiftKind,
	pub shift: u8,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Instruction {
	pub op: BinOp,
	pub dest: Register,
	pub lhs: Register,
	pub rhs: Register,
	pub shift: Shift,
}

bitfield! {
	struct BitField(u32);
	impl Debug;
	pub kind, set_kind : 31, 28;
	pub op, set_op : 27, 23;
	pub rd, set_rd : 22, 18;
	pub rs, set_rs : 17, 13;
	pub rq, set_rq : 12, 8;
	pub shift_kind, set_shift_kind : 7, 5;
	pub shift, set_shift : 4, 0;
}

impl Encode for Instruction {
	fn decode(value: u32) -> Option<Self> {
		let bitfield = BitField(value);
		let kind = Kind::from_u32(bitfield.kind())?;

		if kind != Kind::Rrr {
			return None;
		}

		Some(Instruction {
			op: BinOp::from_u32(bitfield.op())?,
			dest: Register::new(bitfield.rd() as u8).unwrap(),
			lhs: Register::new(bitfield.rs() as u8).unwrap(),
			rhs: Register::new(bitfield.rq() as u8).unwrap(),
			shift: Shift {
				kind: ShiftKind::from_u8(bitfield.shift_kind() as u8)?,
				shift: bitfield.shift() as u8,
			}
		})
	}

	fn encode(&self) -> u32 {
		let mut bitfield = BitField(0);

		bitfield.set_kind(Kind::Rrr.encode());
		bitfield.set_op(self.op.encode());
		bitfield.set_rd(self.dest.encode());
		bitfield.set_rs(self.lhs.encode());
		bitfield.set_rq(self.rhs.encode());
		bitfield.set_shift_kind(self.shift.kind.encode());
		bitfield.set_shift(self.shift.shift as u32);
		bitfield.0
	}
}