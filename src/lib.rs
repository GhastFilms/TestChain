extern crate bigint;
extern crate ring;
use bigint::uint::U256;
use ring::digest::{digest, Algorithm, Context, Digest, SHA256};
use std::{boxed::Box, io::Read};

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

pub fn merkel_root<T: Read + Clone>(
    mut i: Vec<T>,
    algorithm: &'static Algorithm,
) -> Result<Digest, ()> {
    if i.is_empty() {
        println!("no values provided");
        return Err(());
    }
    if i.len() == 1 {
        Ok(digest(
            algorithm,
            &i.get(0)
                .unwrap()
                .clone()
                .bytes()
                .fold(Vec::new(), |mut x, y| {
                    x.push(y.unwrap());
                    x
                }),
        ))
    } else {
        let w: usize = (i.len() as f64 / 2.0).floor() as usize;
        let snd: Vec<T> = i.split_off(w);
        let mut combined_vec: _ = merkel_root(i, algorithm).unwrap().as_ref().to_vec();
        combined_vec.extend_from_slice(merkel_root(snd, algorithm).unwrap().as_ref());
        Ok(digest(algorithm, &combined_vec))
    }
}
