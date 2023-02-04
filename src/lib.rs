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
	Cmp,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Instruction {
	Custom(custom::Instruction),
	Memory(memory::Instruction),
	Rrr(rrr::Instruction),
	Rri(rri::Instruction),
	State(state::Instruction)
}