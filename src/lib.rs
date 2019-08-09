extern crate ring;
extern crate serde;
extern crate serde_derive;
extern crate bincode;

use ring::digest::{digest, Algorithm, Context, Digest, SHA256};
use std::{boxed::Box, io::Read};

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};


use serde::{Serialize, Deserialize};

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

pub fn merkle_root<T: Serialize>(i: &Vec<T>) -> Result<Digest, ()> {
    if i.is_empty() {
        println!("no values provided");
        Err(())
    } else {
        let d: Vec<&T> = i.iter().collect();
        Ok(_merkle_root(d))
    }
}

fn _merkle_root<T: Serialize>(mut i: Vec<&T>) -> Digest {
    if i.len() == 1 {
        calculate_hash(&i.get(0))
    } else {
        let snd: Vec<&T> = i.split_off((i.len() as f64 / 2.0).floor() as usize);
        let mut x = _merkle_root(i).as_ref().to_vec();
        x.extend_from_slice(_merkle_root(snd).as_ref());
        calculate_hash(&x)
    }
}


fn calculate_hash<T: Serialize>(t: &T) -> Digest {
    let bytes = any_as_u8_slice(t);
    digest(&SHA256, &bytes)
}

fn any_as_u8_slice<T: Sized + Serialize>(p: &T) -> Vec<u8> {
    bincode::serialize(&p).unwrap()
}