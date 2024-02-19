/* Copyright 2024 Robert Zieba, see LICENSE file for full license. */
use super::CSR_BLOCK_SIZE;
pub const PSR_BASE: u32 = 0x000;
pub const PSR_SIZE: u32 = 1 * CSR_BLOCK_SIZE;

pub const PSR_PSR0_REG: u32 = 0x000;
pub const CSR_PSR_REG: u32 = PSR_PSR0_REG;
