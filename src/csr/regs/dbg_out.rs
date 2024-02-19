/* Copyright 2024 Robert Zieba, see LICENSE file for full license. */
use super::CSR_BLOCK_SIZE;
pub const DBG_OUT_BASE: u32 = 0x100;
pub const DBG_OUT_SIZE: u32 = 4 * CSR_BLOCK_SIZE;

pub const DBG_OUT_STATUS_REG: u32 = 0x100;
pub const DBG_OUT_CHAR_OUT0_REG: u32 = 0x140;
pub const DBG_OUT_CHAR_IN0_REG: u32 = 0x141;
pub const DBG_OUT_BYTE_OUT0_REG: u32 = 0x180;
pub const DBG_OUT_BYTE_OUT1_REG: u32 = 0x181;
pub const DBG_OUT_BYTE_OUT2_REG: u32 = 0x182;
pub const DBG_OUT_BYTE_OUT3_REG: u32 = 0x183;
pub const DBG_OUT_GPIO_OUT0_REG: u32 = 0x1C0;
pub const DBG_OUT_GPIO_IN0_REG: u32 = 0x1C4;
