extern crate sha;
extern crate bigint;
use bigint::uint::U256;
use sha::{sha256::Sha256, core::hash::Hasher};
use std::box::Box;

struct Block {
    Data: BlockData,
    Header: UnminedBlockHeader,
    TransactionCount: u32,
    MagicNum: u32,
    Size: u32,
}

impl Block {
    pub new() -> Self {
        Block {
            data: String::new("block");
        }
    }
}

Struct BlockData {
    
}


struct UnminedBlockHeader {
    blockref: Box<&Block>
    version: u32,
    prev_hash: U256,
    target: u32,
    merkle_root: U256,
    time: u32;
}




struct MinedBlockHeader {
    version: u32,
    prev_hash: U256,
    target: u32,
    merkle_root: U256,
    time: u32;
}