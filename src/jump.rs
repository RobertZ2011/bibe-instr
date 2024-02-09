use bitfield::bitfield;

use crate::{
	Encode,
	Kind,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Instruction {
	pub imm: i32
}

bitfield! {
	struct Bitfield(u32);
	impl Debug;
	kind, set_kind : 31, 30;
	imm, set_imm : 29, 0;
}

impl Encode for Instruction {
	fn decode(value: u32) -> Option<Self> {
		let bitfield = Bitfield(value);
		Some(Instruction {
			imm: (bitfield.imm() << 2) as i32,
		})
	}

	fn encode(&self) -> u32 {
		let mut bitfield = Bitfield(Kind::Jump.encode());
		bitfield.set_imm((self.imm as u32) >> 2);
		bitfield.0
	}
}