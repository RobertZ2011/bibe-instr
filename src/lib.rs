/* Copyright 2023 Robert Zieba, see LICENSE file for full license. */
use bitfield::bitfield;
use log::debug;
use num_derive::{ FromPrimitive, ToPrimitive };
use num_traits::{ FromPrimitive, ToPrimitive };

pub mod asm;
pub mod misc;
pub mod memory;
pub mod rrr;
pub mod rri;
pub mod csr;
pub mod util;
pub mod jump;

mod register;
mod shift;

pub use register::*;

pub use rri::Condition;
pub use util::Width;

pub use shift::Kind as ShiftKind;
pub use shift::Shift;

pub trait Encode where Self: Sized {
	fn decode(value: u32) -> Option<Self>;
	fn encode(&self) -> u32;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum BinOp {
	Add,
	Sub,

	Mul,
	Div,
	Mod,

	And,
	Or,
	Xor,

	Shl,
	Shr,
	Asl,
	Asr,
	Rol,
	Ror,

	Not,
	Neg,
	
	Addcc,
	Subcc,
}

impl Encode for BinOp {
	fn decode(value: u32) -> Option<BinOp> {
		BinOp::from_u32(value)
	}

	fn encode(&self) -> u32 {
		self.to_u32().unwrap()
	}
}

impl BinOp {
	pub fn is_cc(&self) -> bool {
		match self {
			BinOp::Addcc
			| BinOp::Subcc => true,
			_ => false,
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoadStore {
	Load,
	Store,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LoadStoreOp {
	pub op: LoadStore,
	pub width: Width,
}

impl LoadStoreOp {
	pub fn is_load(&self) -> bool {
		self.op == LoadStore::Load
	}

	pub fn is_store(&self) -> bool {
		self.op == LoadStore::Store
	}
}

impl Encode for LoadStoreOp {
	fn decode(value: u32) -> Option<Self> {
		let op = if value & 0x4 == 0 { LoadStore::Load } else { LoadStore::Store };
		let width = Width::from_u32(value)?;

		Some(LoadStoreOp {
			op,
			width,
		})
	}

	fn encode(&self) -> u32 {
		let encoded = self.width.to_u32().unwrap();
		match self.op {
			LoadStore::Load => encoded,
			LoadStore::Store => encoded | 0x4,
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Instruction {
	Memory(memory::Instruction),
	Csr(csr::Instruction),
	Rrr(rrr::Instruction),
	Rri(rri::Instruction),
	Jump(jump::Instruction),
	Reserved0010(misc::Reserved0010),
	Reserved0011(misc::Reserved0011),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum Kind {
	MemoryRr,
	MemoryRi,
	Csr,
	Rrr,
	Rri,
	Jump,
	Reserved0010,
	Reserved0011
}

bitfield! {
	struct KindBitfield(u32);
	impl Debug;
	pub high, set_high : 31, 30;
	pub sub, set_sub : 29, 28;
	pub discrim, set_discrim: 27, 27;
}

const RR_HIGH: u32 = 0x0;
const RR_RRR_SUB: u32 = 0x0;
const RR_MEM_CSR_SUB: u32 = 0x1;
const RR_MEM_DISCRIM: u32 = 0x0;
const RR_CSR_DISCRIM: u32 = 0x1;
const RESERVED0010_SUB: u32 = 0x2;
const RESERVED0011_SUB: u32 = 0x3;

const RRI_HIGH: u32 = 0x1;
const RI_MEM_HIGH: u32 = 0x2;
const JUMP_HIGH: u32 = 0x3;

impl Encode for Kind {
	fn decode(value: u32) -> Option<Self> {
		let bitfield = KindBitfield(value);
		match bitfield.high() {
			RR_HIGH => match bitfield.sub() {
				RR_RRR_SUB => Some(Kind::Rrr),
				RR_MEM_CSR_SUB => {
					match bitfield.discrim() {
						RR_MEM_DISCRIM => Some(Kind::MemoryRr),
						RR_CSR_DISCRIM => Some(Kind::Csr),
						_ => None,
					}
				},
				_ => None, 
			},
			RRI_HIGH => Some(Kind::Rri),
			RI_MEM_HIGH => Some(Kind::MemoryRi),
			JUMP_HIGH => Some(Kind::Jump),
			_ => None,
		}
	}

	fn encode(&self) -> u32 {
		let mut bitfield = KindBitfield(0);
		match self {
			Kind::MemoryRr => {
				bitfield.set_high(RR_HIGH);
				bitfield.set_sub(RR_MEM_CSR_SUB);
				bitfield.set_discrim(RR_MEM_DISCRIM);
			},
			Kind::MemoryRi => {
				bitfield.set_high(RI_MEM_HIGH);
			},
			Kind::Csr => {
				bitfield.set_high(RR_HIGH);
				bitfield.set_sub(RR_MEM_CSR_SUB);
				bitfield.set_discrim(RR_CSR_DISCRIM);
			},
			Kind::Rrr => {
				bitfield.set_high(RR_HIGH);
				bitfield.set_sub(RR_RRR_SUB);
			},
			Kind::Rri => {
				bitfield.set_high(RRI_HIGH);
			},
			Kind::Jump => {
				bitfield.set_high(JUMP_HIGH);
			},
			Kind::Reserved0010 => {
				bitfield.set_high(RESERVED0010_SUB);
			},
			Kind::Reserved0011 => {
				bitfield.set_high(RESERVED0011_SUB);
			}
		};

		bitfield.0
	}
}

impl Encode for Instruction {
	fn decode(value: u32) -> Option<Self> {
		let kind_res = Kind::decode(value);
		if kind_res.is_none() {
			debug!("Invalid instruction kind");
			return None;
		}

		debug!("Kind {:?}", kind_res.unwrap());

		match kind_res.unwrap() {
			Kind::MemoryRr => Some(Instruction::Memory(memory::Instruction::decode(value)?)),
			Kind::MemoryRi => Some(Instruction::Memory(memory::Instruction::decode(value)?)),
			Kind::Csr => Some(Instruction::Csr(csr::Instruction::decode(value)?)),
			Kind::Rrr => Some(Instruction::Rrr(rrr::Instruction::decode(value)?)),
			Kind::Rri => Some(Instruction::Rri(rri::Instruction::decode(value)?)),
			Kind::Jump => Some(Instruction::Jump(jump::Instruction::decode(value)?)),

			Kind::Reserved0010 => Some(Instruction::Reserved0010(misc::Reserved0010::decode(value)?)),
			Kind::Reserved0011 => Some(Instruction::Reserved0011(misc::Reserved0011::decode(value)?))
		}
	}

	fn encode(&self) -> u32 {
		match self {
			Instruction::Memory(i) => i.encode(),
			Instruction::Csr(i) => i.encode(),
			Instruction::Rrr(i) => i.encode(),
			Instruction::Rri(i) => i.encode(),
			Instruction::Jump(i) => i.encode(),

			Instruction::Reserved0010(i) => i.encode(),
			Instruction::Reserved0011(i) => i.encode(),
		}
	}
}