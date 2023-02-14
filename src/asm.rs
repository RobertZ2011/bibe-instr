/* Copyright 2023 Robert Zieba, see LICENSE file for full license. */
use crate::{
    BinOp,
    Instruction,
    Register,
    rrr,
    rrr::{ 
        Shift,
        ShiftKind,
    },
};

pub fn add_rs(rd :Register, rs: Register, rq: Register, shift: Option<Shift>) -> Instruction {
    Instruction::Rrr(rrr::Instruction {
        op: BinOp::Add,
        dest: rd,
        lhs: rs,
        rhs: rq,
        shift: if shift.is_none() {
            Shift {
                kind: ShiftKind::Shl,
                shift: 0,
            }
        } else {
            shift.unwrap()
        }
    })
}

pub fn add_r(rd :Register, rs: Register, rq: Register) -> Instruction {
    add_rs(rd, rs, rq, None)
}

pub fn nop() -> Instruction {
    add_r(Register::r0(), Register::r0(), Register::r1())
}