extern crate bigint;
extern crate ring;
use bigint::uint::U256;
use ring::digest::{digest, Algorithm, Context, Digest, SHA256};
use std::{boxed::Box, io::Read};

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

struct Block {
    Data: BlockData,
    Header: UnminedBlockHeader,
    TransactionCount: u32,
    MagicNum: u32,
    Size: u32,
}
/*
impl Block {
    pub fn new() -> Self {
        Block {

            Data: vec!(String::new("block"), String::new("block"), String::new("block"), String::new("block"), String::new("block")),
        }
    }
}
*/

struct BlockData {
    data: Vec<Box<dyn Read>>,
}

struct UnminedBlockHeader {
    //blockref: Box<&'a Block>,
    version: u32,
    prev_hash: Digest,
    target: u32,
    merkle_root: Option<Digest>,
    time: u32,
}

struct MinedBlockHeader {
    version: u32,
    prev_hash: Digest,
    target: u32,
    merkle_root: Digest,
    time: u32,
}

pub fn merkle_root<T: Hash>(i: &Vec<T>) -> Result<u64, ()> {
    if i.is_empty() {
        println!("no values provided");
        Err(())
    } else {
        let d: Vec<&T> = i.iter().collect();
        Ok(_merkle_root(d))
    }
}

fn _merkle_root<T: Hash>(mut i: Vec<&T>) -> u64 {
    if i.len() == 1 {
        calculate_hash(&i.get(0))
    } else {
        let w: usize = (i.len() as f64 / 2.0).floor() as usize;
        let snd: Vec<&T> = i.split_off(w);
        let mut combined: _ = _merkle_root(i).to_string();
        combined.push_str(&(_merkle_root(snd).to_string()));
        calculate_hash(&combined)
    }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
