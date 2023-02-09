use num_derive::{ FromPrimitive, ToPrimitive };
use num_traits::{ FromPrimitive, ToPrimitive };
use bitfield::bitfield;

use crate::{
	Encode,
	Kind,
	Register,
	util:: {
		sign_contract,
		sign_extend,
	},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum Operation {
	LoadB,
	StoreB,
	LoadS,
	StoreS,
	LoadW,
	StoreW,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum OpType {
	Load,
	Store,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum Width {
	Byte,
	Short,
	Word,
}

impl Operation {
	pub fn parts(&self) -> (OpType, Width) {
		match self {
			Operation::LoadB => (OpType::Load, Width::Byte),
			Operation::StoreB => (OpType::Store, Width::Byte),
			Operation::LoadS => (OpType::Load, Width::Short),
			Operation::StoreS => (OpType::Store, Width::Short),
			Operation::LoadW => (OpType::Load, Width::Word),
			Operation::StoreW => (OpType::Store, Width::Word),
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Instruction {
	pub op: Operation,
	pub dest: Register,
	pub src: Register,
	pub shift: u8,
	pub immediate: i16,
}

bitfield! {
	struct Bitfield(u32);
	impl Debug;
	pub kind, set_kind : 31, 30;
	pub op, set_op : 29, 25;
	pub dest, set_dest : 24, 20;
	pub src, set_src : 19, 14;
	pub shift, set_shift : 13, 10;
	pub imm, set_imm : 9, 0;
}

impl Encode for Instruction {
	fn decode(value: u32) -> Option<Instruction> {
		let bitfield = Bitfield(value);
		let kind = Kind::from_u32(bitfield.kind())?;
	
		if kind != Kind::Memory {
			return None;
		}

		Some(Instruction {
			op: Operation::from_u32(bitfield.op())?,
			dest: Register::new(bitfield.dest() as u8).unwrap(),
			src: Register::new(bitfield.src() as u8).unwrap(),
			shift: bitfield.shift() as u8,
			immediate: sign_extend(bitfield.imm(), 12) as i16,
		})
	}

	fn encode(&self) -> u32 {
		let mut bitfield = Bitfield(Kind::Memory.encode());
		bitfield.set_op(Operation::to_u32(&self.op).unwrap());
		bitfield.set_dest(self.dest.encode());
		bitfield.set_src(self.src.encode());
		bitfield.set_shift(self.shift as u32);
		bitfield.set_imm(sign_contract(self.immediate as i32, 10));
		bitfield.0
	}
}