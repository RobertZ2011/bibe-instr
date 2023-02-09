use bitfield::bitfield;
use log::debug;
use num_derive::{ FromPrimitive, ToPrimitive };
use num_traits::{ FromPrimitive, ToPrimitive };

pub mod custom;
pub mod memory;
pub mod rrr;
pub mod rri;
pub mod model;
pub mod util;

pub trait Encode where Self: Sized {
	fn decode(value: u32) -> Option<Self>;
	fn encode(&self) -> u32;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Register(u8);

impl Register {
	pub fn new(reg: u8) -> Option<Register> {
		if reg > 31 {
			None
		} else {
			Some(Register(reg))
		}
	}

	pub fn as_u8(self) -> u8 {
		self.0
	}

	pub fn r0() -> Register {
		Register(0)
	}

	pub fn r1() -> Register {
		Register(1)
	}

	pub fn r2() -> Register {
		Register(2)
	}

	pub fn r3() -> Register {
		Register(3)
	}

	pub fn r4() -> Register {
		Register(4)
	}

	pub fn r5() -> Register {
		Register(5)
	}

	pub fn r6() -> Register {
		Register(6)
	}

	pub fn r7() -> Register {
		Register(7)
	}

	pub fn r8() -> Register {
		Register(8)
	}

	pub fn r9() -> Register {
		Register(9)
	}

	pub fn r10() -> Register {
		Register(10)
	}

	pub fn r11() -> Register {
		Register(11)
	}

	pub fn r12() -> Register {
		Register(12)
	}

	pub fn r13() -> Register {
		Register(13)
	}

	pub fn r14() -> Register {
		Register(14)
	}

	pub fn r15() -> Register {
		Register(15)
	}

	pub fn r16() -> Register {
		Register(16)
	}

	pub fn r17() -> Register {
		Register(17)
	}

	pub fn r18() -> Register {
		Register(18)
	}

	pub fn r19() -> Register {
		Register(19)
	}

	pub fn r20() -> Register {
		Register(20)
	}

	pub fn r21() -> Register {
		Register(21)
	}

	pub fn r22() -> Register {
		Register(22)
	}

	pub fn r23() -> Register {
		Register(23)
	}

	pub fn r24() -> Register {
		Register(24)
	}

	pub fn r25() -> Register {
		Register(25)
	}

	pub fn r26() -> Register {
		Register(26)
	}

	pub fn r27() -> Register {
		Register(27)
	}

	pub fn r28() -> Register {
		Register(28)
	}

	pub fn r29() -> Register {
		Register(29)
	}

	pub fn r30() -> Register {
		Register(30)
	}

	pub fn r31() -> Register {
		Register(31)
	}

	pub fn pc() -> Register {
		Self::r31()
	}

	pub fn lr() -> Register {
		Self::r30()
	}

	pub fn fp() -> Register {
		Self::r29()
	}

	pub fn sp() -> Register {
		Self::r28()
	}
}

impl Encode for Register {
	fn decode(value: u32) -> Option<Register> {
		Register::new(value as u8)
	}

	fn encode(&self) -> u32 {
		self.as_u8() as u32
	}
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
	Cmp,
}

impl Encode for BinOp {
	fn decode(value: u32) -> Option<BinOp> {
		BinOp::from_u32(value)
	}

	fn encode(&self) -> u32 {
		self.to_u32().unwrap()
	}
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Instruction {
	Custom(custom::Instruction),
	Memory(memory::Instruction),
	Model(model::Instruction),
	Rrr(rrr::Instruction),
	Rri(rri::Instruction),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum Kind {
	Custom,
	Memory,
	Model,
	Rrr,
	Rri,
}

bitfield! {
	struct KindBitfield(u32);
	impl Debug;
	pub high, set_high : 31, 30;
	pub sub, set_sub : 29, 28;
}

const MMR_HIGH: u32 = 0x0;
const MEMORY_SUB: u32 = 0x1;
const MODEL_SUB: u32 = 0x2;
const RRR_SUB: u32 = 0x0;

const RRI_HIGH: u32 = 0x1;
const CUSTOM_HIGH: u32 = 0x3;

impl Encode for Kind {
	fn decode(value: u32) -> Option<Self> {
		let bitfield = KindBitfield(value);
		match bitfield.high() {
			MMR_HIGH => match bitfield.sub() {
				RRR_SUB => Some(Kind::Rrr),
				MEMORY_SUB => Some(Kind::Memory),
				MODEL_SUB => Some(Kind::Model),
				_ => None, 
			},
			RRI_HIGH => Some(Kind::Rri),
			CUSTOM_HIGH => Some(Kind::Custom),
			_ => None,
		}
	}

	fn encode(&self) -> u32 {
		let mut bitfield = KindBitfield(0);
		
		match self {
			Kind::Custom => bitfield.set_high(CUSTOM_HIGH),
			Kind::Memory => bitfield.set_sub(MEMORY_SUB),
			Kind::Model => bitfield.set_sub(MODEL_SUB),
			Kind::Rrr => bitfield.set_high(MMR_HIGH),
			Kind::Rri => bitfield.set_high(RRI_HIGH),
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

		match kind_res.unwrap() {
			//Kind::Custom => custom::Instruction::decode(value),
			Kind::Memory => Some(Instruction::Memory(memory::Instruction::decode(value)?)),
			//Kind::Model => model::Instruction::decode(value),
			Kind::Rrr => Some(Instruction::Rrr(rrr::Instruction::decode(value)?)),
			Kind::Rri => Some(Instruction::Rri(rri::Instruction::decode(value)?)),
			_ => None,
		}
	}

	fn encode(&self) -> u32 {
		match self {
			//Instruction::Custom(i) => i.encode(),
			//Instruction::Memory(i) => i.encode(),
			//Instruction::Model(i) => i.encode(),
			Instruction::Rrr(i) => i.encode(),
			Instruction::Rri(i) => i.encode(),
			_ => panic!("Unsupported instruction type"),
		}
	}
}