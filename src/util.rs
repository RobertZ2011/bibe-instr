pub const fn sign_contract(val: i32, target: u8) -> u32 {
	let shift = 32 - target;
	let unsigned = (val as u32) << shift;
	unsigned >> shift
}

pub const fn sign_extend(val: u32, width: u8) -> i32 {
	let shift = 32 - width;
	let signed = (val << shift) as i32;
	signed >> shift
}