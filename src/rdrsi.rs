use num_traits::{ FromPrimitive, ToPrimitive };
use super::{
	Register,
	RegOp,
	Encode,
};

const OP_OFFSET: u8 = 23;
const DEST_OFFSET: u8 = 18;
const SRC_OFFSET: u8 = 14;

#[derive(Clone, Copy, Debug)]
pub struct Instruction {
	pub op: RegOp,
	pub dest: Register,
	pub src: Register,
	pub imm: u32,
}

impl Encode for Instruction {
	fn encode(&self) -> u32 {
		ToPrimitive::to_u32(&self.op).unwrap() << OP_OFFSET
		| ToPrimitive::to_u32(&self.dest).unwrap() << DEST_OFFSET
		| ToPrimitive::to_u32(&self.src).unwrap() << SRC_OFFSET
		| (self.imm & 0x3FFF)
	}

	fn decode(value: u32) -> Self {
		Instruction {
			op: FromPrimitive::from_u32(value >> OP_OFFSET).unwrap(),
			dest: FromPrimitive::from_u32(value >> DEST_OFFSET & 0x1F).unwrap(),
			src: FromPrimitive::from_u32(value >> SRC_OFFSET & 0x1F).unwrap(),
			imm: value & 0x3FFF,
		}
	}
}