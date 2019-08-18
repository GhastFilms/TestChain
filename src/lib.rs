#![allow(dead_code)]

extern crate ring;
//#[macro_use]
//extern crate serde_derive;

use ring::{
    digest::{digest, Digest, SHA256},
};
//use serde::Serialize;

pub mod transaction;
use transaction::TransactionContainer;

const VERSION: u32 = 1; 

pub struct Block {
    header: BlockHeader,
    transactions: TransactionContainer,
    size: u32,
}

// u8s and u32s are hashes with .to_le_bytes()
pub struct BlockHeader {
    version: u32,
    prev_hash: Digest,
    target: u32,
    merkle_root: Digest,
    time: u32,
    nonce: u64,
}

