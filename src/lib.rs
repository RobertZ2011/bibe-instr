use num_derive::{ FromPrimitive, ToPrimitive };
use num_traits::{ FromPrimitive, ToPrimitive };

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Register(u8);

#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum Condition {
	Al,
	Eq,
	Ne,
	Gt,
	Ge,
	Lt,
	Le,
}

#[derive(Clone, Copy, Debug, FromPrimitive, ToPrimitive)]
pub enum BinOp {
	Add,
	Sub,
	Mul,
	Div,

	And,
    Or,
    Xor

	ShiftL,
    ShiftR,
    AshiftL,
    AshiftR,
}

#[derive(Clone, Copy, Debug, FromPrimitive, ToPrimitive)]
pub enum Kind {
	Rrr,
	Memory,
	State,
	Unary,
	Rri,
	Rric,
	ImplDefined,
}