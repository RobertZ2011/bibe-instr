use std::ops::Add;

/* Copyright 2023 Robert Zieba, see LICENSE file for full license. */
use num_derive::{ FromPrimitive, ToPrimitive };

pub const fn sign_contract(val: i32, target: u8) -> u32 {
	let shift = 32 - target;
	let unsigned = (val as u32) << shift;
	unsigned >> shift
}

pub const fn sign_extend(val: u32, width: i8) -> u32 {
	let shift = 32 - width;
	let signed = (val << shift) as i32;
	(signed >> shift) as u32
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum Width {
	Byte,
	Short,
	Word,
}

impl Width {
	pub fn to_len(self) -> u32 {
		match self {
			Width::Byte => 1,
			Width::Short => 2,
			Width::Word => 4,
		}
	}

	pub fn to_mask(self) -> u32 {
		match self {
			Width::Byte => 0xff,
			Width::Short => 0xffff,
			Width::Word => 0xffffffff,
		}
	}
}

impl Add<Width> for u32 {
	type Output = u32;

	fn add(self, rhs: Width) -> Self::Output {
		self + rhs.to_len()
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn extend() {
		assert_eq!(sign_extend(0x9A, 8), 0xFFFFFF9A);
		assert_eq!(sign_extend(0x7AA, 11), 0xFFFFFFAA);
		assert_eq!(sign_extend(0xAAAA, 16), 0xFFFFAAAA);
		assert_eq!(sign_extend(0xCAAAAA, 24), 0xFFCAAAAA);
	}

	#[test]
	fn contract() {
		assert_eq!(sign_contract(-102, 8), 0x9A);
		assert_eq!(sign_contract(-86, 11), 0x7AA);
		assert_eq!(sign_contract(-3495254, 16), 0xAAAA);
		assert_eq!(sign_contract(-3495254, 24), 0xCAAAAA);
	}

	#[test]
	fn add_width() {
		assert_eq!(4 + Width::Byte, 5);
		assert_eq!(4 + Width::Short, 6);
		assert_eq!(4 + Width::Word, 8);
	}

	#[test]
	fn width_to_mask() {
		assert_eq!(Width::to_mask(Width::Byte), 0xff);
		assert_eq!(Width::to_mask(Width::Short), 0xffff);
		assert_eq!(Width::to_mask(Width::Word), 0xffffffff);
	}
}