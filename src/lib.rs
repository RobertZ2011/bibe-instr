use num_derive::{ FromPrimitive, ToPrimitive };

pub mod custom;
pub mod memory;
pub mod rrr;
pub mod rri;
pub mod state;

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
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum BinOp {
	Add,
	Sub,
	Mul,
	Div,

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
	Sxt,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Instruction {
	Custom(custom::Instruction),
	Memory(memory::Instruction),
	Rrr(rrr::Instruction),
	Rri(rri::Instruction),
	State(state::Instruction)
}