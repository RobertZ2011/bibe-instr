/* Copyright 2023 Robert Zieba, see LICENSE file for full license. */
use crate::Encode;

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
		Self(0)
	}

	pub fn r1() -> Register {
		Self(1)
	}

	pub fn r2() -> Register {
		Self(2)
	}

	pub fn r3() -> Register {
		Self(3)
	}

	pub fn r4() -> Register {
		Self(4)
	}

	pub fn r5() -> Register {
		Self(5)
	}

	pub fn r6() -> Register {
		Self(6)
	}

	pub fn r7() -> Register {
		Self(7)
	}

	pub fn r8() -> Register {
		Self(8)
	}

	pub fn r9() -> Register {
		Self(9)
	}

	pub fn r10() -> Register {
		Self(10)
	}

	pub fn r11() -> Register {
		Self(11)
	}

	pub fn r12() -> Register {
		Self(12)
	}

	pub fn r13() -> Register {
		Self(13)
	}

	pub fn r14() -> Register {
		Self(14)
	}

	pub fn r15() -> Register {
		Self(15)
	}

	pub fn r16() -> Register {
		Self(16)
	}

	pub fn r17() -> Register {
		Self(17)
	}

	pub fn r18() -> Register {
		Self(18)
	}

	pub fn r19() -> Register {
		Self(19)
	}

	pub fn r20() -> Register {
		Self(20)
	}

	pub fn r21() -> Register {
		Self(21)
	}

	pub fn r22() -> Register {
		Self(22)
	}

	pub fn r23() -> Register {
		Self(23)
	}

	pub fn r24() -> Register {
		Self(24)
	}

	pub fn r25() -> Register {
		Self(25)
	}

	pub fn r26() -> Register {
		Self(26)
	}

	pub fn r27() -> Register {
		Self(27)
	}

	pub fn r28() -> Register {
		Self(28)
	}

	pub fn r29() -> Register {
		Self(29)
	}

	pub fn r30() -> Register {
		Self(30)
	}

	pub fn r31() -> Register {
		Self(31)
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

	pub fn z() -> Register {
		Self::r0()
	}

	pub fn arg(reg: u8) -> Option<Register> {
		if reg <= 8 {
			Some(Self(1 + reg))
		} else {
			None
		}
	}

	pub fn out(reg: u8) -> Option<Register> {
		if reg <= 5 {
			Some(Self(10 + reg))
		} else {
			None
		}
	}

	pub fn local(reg: u8) -> Option<Register> {
		if reg <= 3 {
			Some(Self(16 + reg))
		} else {
			None
		}
	}

	pub fn temp(reg: u8) -> Option<Register> {
		if reg <= 7 {
			Some(Self(20 + reg))
		} else {
			None
		}
	}

	pub fn a0() -> Register {
		Self::arg(0).unwrap()
	}

	pub fn a1() -> Register {
		Self::arg(1).unwrap()
	}

	pub fn a2() -> Register {
		Self::arg(2).unwrap()
	}

	pub fn a3() -> Register {
		Self::arg(3).unwrap()
	}

	pub fn a4() -> Register {
		Self::arg(4).unwrap()
	}

	pub fn a5() -> Register {
		Self::arg(5).unwrap()
	}

	pub fn a6() -> Register {
		Self::arg(6).unwrap()
	}

	pub fn a7() -> Register {
		Self::arg(7).unwrap()
	}

	pub fn a8() -> Register {
		Self::arg(8).unwrap()
	}

	pub fn o0() -> Register {
		Self::out(0).unwrap()
	}

	pub fn o1() -> Register {
		Self::out(1).unwrap()
	}

	pub fn o2() -> Register {
		Self::out(2).unwrap()
	}

	pub fn o3() -> Register {
		Self::out(3).unwrap()
	}

	pub fn o4() -> Register {
		Self::out(4).unwrap()
	}

	pub fn o5() -> Register {
		Self::out(5).unwrap()
	}

	pub fn l0() -> Register {
		Self::local(0).unwrap()
	}

	pub fn l1() -> Register {
		Self::local(1).unwrap()
	}

	pub fn l2() -> Register {
		Self::local(2).unwrap()
	}

	pub fn l3() -> Register {
		Self::local(3).unwrap()
	}

	pub fn l4() -> Register {
		Self::local(4).unwrap()
	}

	pub fn l5() -> Register {
		Self::local(5).unwrap()
	}

	pub fn l6() -> Register {
		Self::local(6).unwrap()
	}

	pub fn l7() -> Register {
		Self::local(7).unwrap()
	}

	pub fn t0() -> Register {
		Self::temp(0).unwrap()
	}

	pub fn t1() -> Register {
		Self::temp(1).unwrap()
	}

	pub fn t2() -> Register {
		Self::temp(2).unwrap()
	}

	pub fn t3() -> Register {
		Self::temp(3).unwrap()
	}

	pub fn t4() -> Register {
		Self::temp(4).unwrap()
	}

	pub fn t5() -> Register {
		Self::temp(5).unwrap()
	}

	pub fn t6() -> Register {
		Self::temp(6).unwrap()
	}

	pub fn t7() -> Register {
		Self::temp(7).unwrap()
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

#[cfg(test)]
mod test {
	use super::*;

	// New
	#[test]
	fn new_err() {
		assert!(Register::new(32).is_none());
	}

	#[test]
	fn new_range() {
		assert_eq!(Register::new(0).unwrap().0, 0);
		assert_eq!(Register::new(31).unwrap().0, 31);
	}

	// Arg
	#[test]
	fn arg_err() {
		assert!(Register::arg(9).is_none());
	}

	#[test]
	fn arg_range() {
		assert_eq!(Register::arg(0).unwrap().0, 1);
		assert_eq!(Register::arg(8).unwrap().0, 9);
	}

	// Out
	#[test]
	fn out_err() {
		assert!(Register::out(6).is_none());
	}

	#[test]
	fn out_range() {
		assert_eq!(Register::out(0).unwrap().0, 10);
		assert_eq!(Register::out(5).unwrap().0, 15);
	}

	// Local
	#[test]
	fn local_err() {
		assert!(Register::local(4).is_none());
	}

	#[test]
	fn local_range() {
		assert_eq!(Register::local(0).unwrap().0, 16);
		assert_eq!(Register::local(3).unwrap().0, 19);
	}

	// Temp
	#[test]
	fn temp_err() {
		assert!(Register::temp(8).is_none());
	}

		#[test]
		fn temp_range() {
			assert_eq!(Register::temp(0).unwrap().0, 20);
			assert_eq!(Register::temp(7).unwrap().0, 27);
		}

	// Special
	#[test]
	fn pc() {
		assert_eq!(Register::pc().0, 31);
	}

	#[test]
	fn lr() {
		assert_eq!(Register::lr().0, 30);
	}

	#[test]
	fn fp() {
		assert_eq!(Register::fp().0, 29);
	}

	#[test]
	fn sp() {
		assert_eq!(Register::sp().0, 28);
	}

	#[test]
	fn z() {
		assert_eq!(Register::z().0, 0);
	}

	// General
	#[test]
	fn r0() {
		assert_eq!(Register::r0().0, 0);
	}

	#[test]
	fn r1() {
		assert_eq!(Register::r1().0, 1);
	}

	#[test]
	fn r2() {
		assert_eq!(Register::r2().0, 2);
	}

	#[test]
	fn r3() {
		assert_eq!(Register::r3().0, 3);
	}

	#[test]
	fn r4() {
		assert_eq!(Register::r4().0, 4);
	}

	#[test]
	fn r5() {
		assert_eq!(Register::r5().0, 5);
	}

	#[test]
	fn r6() {
		assert_eq!(Register::r6().0, 6);
	}

	#[test]
	fn r7() {
		assert_eq!(Register::r7().0, 7);
	}

	#[test]
	fn r8() {
		assert_eq!(Register::r8().0, 8);
	}

	#[test]
	fn r9() {
		assert_eq!(Register::r9().0, 9);
	}

	#[test]
	fn r10() {
		assert_eq!(Register::r10().0, 10);
	}

	#[test]
	fn r11() {
		assert_eq!(Register::r11().0, 11);
	}

	#[test]
	fn r12() {
		assert_eq!(Register::r12().0, 12);
	}

	#[test]
	fn r13() {
		assert_eq!(Register::r13().0, 13);
	}

	#[test]
	fn r14() {
		assert_eq!(Register::r14().0, 14);
	}

	#[test]
	fn r15() {
		assert_eq!(Register::r15().0, 15);
	}

	#[test]
	fn r16() {
		assert_eq!(Register::r16().0, 16);
	}

	#[test]
	fn r17() {
		assert_eq!(Register::r17().0, 17);
	}

	#[test]
	fn r18() {
		assert_eq!(Register::r18().0, 18);
	}

	#[test]
	fn r19() {
		assert_eq!(Register::r19().0, 19);
	}

	#[test]
	fn r20() {
		assert_eq!(Register::r20().0, 20);
	}

	#[test]
	fn r21() {
		assert_eq!(Register::r21().0, 21);
	}

	#[test]
	fn r22() {
		assert_eq!(Register::r22().0, 22);
	}

	#[test]
	fn r23() {
		assert_eq!(Register::r23().0, 23);
	}

	#[test]
	fn r24() {
		assert_eq!(Register::r24().0, 24);
	}

	#[test]
	fn r25() {
		assert_eq![Register::r25().0, 25];
	}

	#[test]
	fn r26() {
		assert_eq![Register::r26().0, 26];
	}

	#[test]
	fn r27() {
		assert_eq!(Register::r27().0, 27);
	}

	#[test]
	fn r28() {
		assert_eq!(Register::r28().0, 28);
	}

	#[test]
	fn r29() {
		assert_eq!(Register::r29().0, 29);
	}

	#[test]
	fn r30() {
		assert_eq!(Register::r30().0, 30);
	}

	#[test]
	fn r31() {
		assert_eq!(Register::r31().0, 31);
	}

	// Args
	#[test]
	fn a0() {
		assert_eq!(Register::a0().0, 1);
	}

	#[test]
	fn a1() {
		assert_eq!(Register::a1().0, 2);
	}

	#[test]
	fn a2() {
		assert_eq!(Register::a2().0, 3);
	}

	#[test]
	fn a3() {
		assert_eq!(Register::a3().0, 4);
	}

	#[test]
	fn a4() {
		assert_eq!(Register::a4().0, 5);
	}

	#[test]
	fn a5() {
		assert_eq!(Register::a5().0, 6);
	}

	#[test]
	fn a6() {
		assert_eq!(Register::a6().0, 7);
	}

	#[test]
	fn a7() {
		assert_eq!(Register::a7().0, 8);
	}

	#[test]
	fn a8() {
		assert_eq!(Register::a8().0, 9);
	}

	// Out
	#[test]
	fn o0() {
		assert_eq!(Register::o0().0, 10);
	}

	#[test]
	fn o1() {
		assert_eq!(Register::o1().0, 11);
	}

	#[test]
	fn o2() {
		assert_eq!(Register::o2().0, 12);
	}

	#[test]
	fn o3() {
		assert_eq!(Register::o3().0, 13);
	}

	#[test]
	fn o4() {
		assert_eq!(Register::o4().0, 14);
	}

	#[test]
	fn o5() {
		assert_eq!(Register::o5().0, 15);
	}

	// Local
	#[test]
	fn l0() {
		assert_eq!(Register::l0().0, 16);
	}

	#[test]
	fn l1() {
		assert_eq!(Register::l1().0, 17);
	}

	#[test]
	fn l2() {
		assert_eq!(Register::l2().0, 18);
	}

	#[test]
	fn l3() {
		assert_eq!(Register::l3().0, 19);
	}

	// Temp
	#[test]
	fn t0() {
		assert_eq!(Register::t0().0, 20);
	}

	#[test]
	fn t1() {
		assert_eq!(Register::t1().0, 21);
	}

	#[test]
	fn t2() {
		assert_eq!(Register::t2().0, 22);
	}

	#[test]
	fn t3() {
		assert_eq!(Register::t3().0, 23);
	}

	#[test]
	fn t4() {
		assert_eq!(Register::t4().0, 24);
	}

	#[test]
	fn t5() {
		assert_eq!(Register::t5().0, 25);
	}

	#[test]
	fn t6() {
		assert_eq!(Register::t6().0, 26);
	}

	#[test]
	fn t7() {
		assert_eq!(Register::t7().0, 27);
	}
}