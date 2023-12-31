use crate::Encode;

/* Copyright 2023 Robert Zieba, see LICENSE file for full license. */
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Custom {
    value: u32
}

impl Encode for Custom {
    fn decode(value: u32) -> Option<Self> {
        todo!()
    }

    fn encode(&self) -> u32 {
        todo!()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Reserved10 {
    value: u32,
}

impl Encode for Reserved10 {
    fn decode(value: u32) -> Option<Self> {
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
    fn decode(value: u32) -> Option<Self> {
        todo!()
    }

    fn encode(&self) -> u32 {
        todo!()
    }
}
