use crate::{
	Encode,
	Kind,
	memory::{
		AddressMode,
		Operation,
		OpType,
	},
	Register,
	Width,
	util::{
		sign_contract,
		sign_extend,
	},
};

use num_traits::{ FromPrimitive, ToPrimitive };
use bitfield::bitfield;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Instruction {
	op: (OpType, Width),
	rd: Register,
	rs: Register,
	imm: i16,
}

bitfield! {
	struct Bitfield(u32);
	impl Debug;
	pub kind, set_kind : 31, 28;
	pub op, set_op : 27, 23;
	pub rd, set_rd : 22, 19;
	pub rs, set_rs : 18, 14;
	pub imm, set_imm : 13, 0;
}

impl Encode for Instruction {
	fn decode(value: u32) -> Option<Self> {
		let bitfield = Bitfield(value);
		let (mode, op, width) = Operation::from_u32(bitfield.op())?.parts()?;
		let kind = Kind::decode(value)?;

		if kind != Kind::Memory || mode != AddressMode::Ri {
				return None;
		}

		Some(Instruction { 
			op: (op, width),
			rd: Register::new(bitfield.rd() as u8).unwrap(),
			rs: Register::new(bitfield.rs() as u8).unwrap(), 
			imm: sign_extend(bitfield.imm(), 14) as i16,
		})
	}

	fn encode(&self) -> u32 {
		let mut bitfield = Bitfield(0);

		let op= Operation::from_parts(AddressMode::Rr, self.op.0, self.op.1).unwrap().to_u32().unwrap();
		bitfield.set_kind(Kind::Memory.to_u32().unwrap());
		bitfield.set_op(op);
		bitfield.set_rd(self.rd.as_u8() as u32);
		bitfield.set_rs(self.rs.as_u8() as u32);
		bitfield.set_imm(sign_contract(self.imm as i32, 14));

		bitfield.0

	}
}