/* Copyright 2023 Robert Zieba, see LICENSE file for full license. */
use crate::Encode;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Reserved0010 {
    value: u32,
}

impl Encode for Reserved0010 {
    fn decode(_value: u32) -> Option<Self> {
        todo!()
    }

    fn encode(&self) -> u32 {
        todo!()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Reserved0011 {
    value: u32,
}

impl Encode for Reserved0011 {
    fn decode(_value: u32) -> Option<Self> {
        todo!()
    }

    fn encode(&self) -> u32 {
        todo!()
    }
}
