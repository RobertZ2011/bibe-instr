/* Copyright 2023 Robert Zieba, see LICENSE file for full license. */
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

#[cfg(test)]
mod test {
	use crate::util::{ sign_extend, sign_contract };

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
}