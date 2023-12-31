use num_derive::{ FromPrimitive, ToPrimitive };
use num_traits::{ FromPrimitive, ToPrimitive };

use crate::Encode;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum Kind {
	#[default]
	Shl,
	Shr,
	Asl,
	Asr,
	Rol,
	Ror,
}

impl Encode for Kind {
	fn decode(value: u32) -> Option<Kind> {
		Kind::from_u32(value)
	}

	fn encode(&self) -> u32 {
		self.to_u32().unwrap()
	}
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Shift {
	pub kind: Kind,
	pub shift: u8,
}