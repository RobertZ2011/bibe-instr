use num_traits::{ FromPrimitive, ToPrimitive };
use super::{
	Register,
	RegOp,
	Encode,
};

const OP_OFFSET: u8 = 23;
const DEST_OFFSET: u8 = 18;
const SRC1_OFFSET: u8 = 14;
const SRC2_OFFSET: u8 = 9;

#[derive(Clone, Copy, Debug)]
pub struct Instruction {
	pub op: RegOp,
	pub dest: Register,
	pub src1: Register,
	pub src2: Register,
}

impl Encode for Instruction {
	fn encode(&self) -> u32 {
		ToPrimitive::to_u32(&self.op).unwrap() << OP_OFFSET
		| ToPrimitive::to_u32(&self.dest).unwrap() << DEST_OFFSET
		| ToPrimitive::to_u32(&self.src1).unwrap() << SRC1_OFFSET
		| ToPrimitive::to_u32(&self.src2).unwrap() << SRC2_OFFSET
	}

	fn decode(value: u32) -> Self {
		Instruction {
			op: FromPrimitive::from_u32(value >> OP_OFFSET).unwrap(),
			dest: FromPrimitive::from_u32(value >> DEST_OFFSET & 0x1F).unwrap(),
			src1: FromPrimitive::from_u32(value >> SRC1_OFFSET & 0x1F).unwrap(),
			src2: FromPrimitive::from_u32(value >> SRC2_OFFSET & 0x1F).unwrap(),
		}
	}
}