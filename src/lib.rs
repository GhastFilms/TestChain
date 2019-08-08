extern crate ring;
extern crate bigint;
use ring::digest::{SHA256, Digest, Context};
use bigint::uint::U256;
use std::{boxed::Box, io::Read};

struct Block {
    Data: BlockData,
    Header: UnminedBlockHeader,
    TransactionCount: u32,
    MagicNum: u32,
    Size: u32,
}

impl Block {
    pub fn new() -> Self {
        Block {
            data: vec!(String::new("block"), String::new("block"), String::new("block"), String::new("block"), String::new("block"), ),
        }
    }
}

struct BlockData {
    data: Vec<dyn Read>,
}

struct UnminedBlockHeader {
    blockref: Box<&'a Block>,
    version: u32,
    prev_hash: U256,
    target: u32,
    merkle_root: U256,
    time: u32,
}




struct MinedBlockHeader {
    version: u32,
    prev_hash: U256,
    target: u32,
    merkle_root: U256,
    time: u32,
}




fn merkel_root(i: Vec<dyn Read>) -> U256 {
    if i.len() = 1 {
        digest::digest(&digest::SHA256, i.get(0).unwrap().bytes())
    }
    if i.len().is_even() {

    }