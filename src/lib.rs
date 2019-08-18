#![allow(dead_code)]
extern crate ring;

use ring::{
    digest::Digest, digest::Context, digest::SHA256
};

pub mod transaction;

use transaction::TransactionContainer;

const VERSION: u32 = 1;

pub struct Block {
    pub header: BlockHeader,
    pub transactions: TransactionContainer,
}

// u8s and u32s are hashes with .to_le_bytes()

// target cannot be longer than 32 bytes
//
pub struct BlockHeader {
    pub version: u32,
    pub prev_hash: Digest,
    pub target: Vec<u8>,
    pub merkle_root: Digest,
    pub time: u64,
    pub nonce: u64,
}

impl BlockHeader {
    pub fn hash(&self, h: &mut Context) {
            h.update(&self.version.to_le_bytes());
            h.update(&self.prev_hash.as_ref());
            h.update(&self.target.as_ref());
            h.update(&self.merkle_root.as_ref());
            h.update(&self.time.to_le_bytes());
    }
}

pub fn pow(b: &mut Block) {
    let mut d: Digest;
    let mut nonce: u64 = 0;
    let target: Vec<u8> = b.header.target.clone();
    if target.len() >= 33 {
        panic!("invalid target lenth");
    }
    //do while :/
    while {
        let mut h = Context::new(&SHA256);
        b.header.hash(&mut h);
        h.update(&nonce.to_le_bytes());
        d = h.finish();
        nonce += 1;
        
        println!("nonce: {}, hash: {:?}", nonce, d);
        
        d.as_ref() <= &target
    } {};

    if d.as_ref() <= &target {
        b.header.nonce = nonce;       
    }
}

struct PartialBlock {
    pub header: Option<BlockHeader>,
    pub transactions: TransactionContainer,
}

impl PartialBlock {
    pub fn new() -> PartialBlock {
        PartialBlock {
            header: None,
            transactions: TransactionContainer::new(),
        }
    }
}

struct PartialBlockHeader {
    pub version: u32,
    pub prev_hash: Digest,
    pub target: Vec<u8>,
    pub merkle_root: Digest,
    pub time: u64,
    pub nonce: Option<u64>,
}

const TARGET: [u8; 32] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];

use std::time::{SystemTime, UNIX_EPOCH};


impl PartialBlockHeader {
    pub fn new(prev_hash: Digest, target: &[u8], merkle_root: Digest) -> PartialBlockHeader {
        PartialBlockHeader {
            time: {
                let s = SystemTime::now(); 
                s.duration_since(UNIX_EPOCH).unwrap().as_secs() 
            },
            version: VERSION, 
            target: target.to_vec(),
            nonce: None,
            prev_hash,
            merkle_root,
        }
    }

    pub fn 



}


