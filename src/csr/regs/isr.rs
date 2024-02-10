/* Copyright 2024 Robert Zieba, see LICENSE file for full license. */
use super::{ CSR_BLOCK_SIZE, PSR_BASE, PSR_SIZE };

pub const ISR_BASE: u32 = PSR_BASE + PSR_SIZE;
pub const ISR_SIZE: u32 = 3 * CSR_BLOCK_SIZE;

pub const ISR_BASE_REG: u32 = ISR_BASE;
pub const ISR_ERR1_REG: u32 = ISR_BASE_REG + 0x4;
pub const ISR_ERR2_REG: u32 = ISR_BASE_REG + 0x8;
pub const ISR_ENTER_REG: u32 = ISR_BASE_REG + 0xc;
pub const ISR_EXIT_REG: u32 = ISR_BASE_REG + 0x10;

pub const ISR_R1_REG: u32 = ISR_BASE_REG + CSR_BLOCK_SIZE;
pub const ISR_R2_REG: u32 = ISR_R1_REG + 0x04;
pub const ISR_R3_REG: u32 = ISR_R1_REG + 0x08;
pub const ISR_R4_REG: u32 = ISR_R1_REG + 0x0c;
pub const ISR_R5_REG: u32 = ISR_R1_REG + 0x10;
pub const ISR_R6_REG: u32 = ISR_R1_REG + 0x14;
pub const ISR_R7_REG: u32 = ISR_R1_REG + 0x18;
pub const ISR_R8_REG: u32 = ISR_R1_REG + 0x1c;
pub const ISR_R9_REG: u32 = ISR_R1_REG + 0x20;
pub const ISR_R10_REG: u32 = ISR_R1_REG + 0x24;
pub const ISR_R11_REG: u32 = ISR_R1_REG + 0x28;
pub const ISR_R12_REG: u32 = ISR_R1_REG + 0x2c;
pub const ISR_R13_REG: u32 = ISR_R1_REG + 0x30;
pub const ISR_R14_REG: u32 = ISR_R1_REG + 0x34;
pub const ISR_R15_REG: u32 = ISR_R1_REG + 0x38;
pub const ISR_R16_REG: u32 = ISR_R1_REG + 0x3c;

pub const ISR_R17_REG: u32 = ISR_BASE_REG + 2 * CSR_BLOCK_SIZE;
pub const ISR_R18_REG: u32 = ISR_R17_REG + 0x04;
pub const ISR_R19_REG: u32 = ISR_R17_REG + 0x08;
pub const ISR_R20_REG: u32 = ISR_R17_REG + 0x0c;
pub const ISR_R21_REG: u32 = ISR_R17_REG + 0x10;
pub const ISR_R22_REG: u32 = ISR_R17_REG + 0x14;
pub const ISR_R23_REG: u32 = ISR_R17_REG + 0x18;
pub const ISR_R24_REG: u32 = ISR_R17_REG + 0x1c;
pub const ISR_R25_REG: u32 = ISR_R17_REG + 0x20;
pub const ISR_R26_REG: u32 = ISR_R17_REG + 0x24;
pub const ISR_R27_REG: u32 = ISR_R17_REG + 0x28;
pub const ISR_SP_REG: u32 = ISR_R17_REG + 0x2c;
pub const ISR_FP_REG: u32 = ISR_R17_REG + 0x30;
pub const ISR_LR_REG: u32 = ISR_R17_REG + 0x34;
pub const ISR_PC_REG: u32 = ISR_R17_REG + 0x38;