
use ring::{
    digest::Digest, digest::Context, digest::SHA256
};

use super::VERSION;
use super::TARGET;
use super::header::*;
use crate::TransactionContainer;

pub struct Block {
    pub header: BlockHeader,
    pub transactions: TransactionContainer,
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
    pub header: Option<PartialBlockHeader>,
    pub transactions: TransactionContainer,
}

impl PartialBlock {
    pub fn new() -> PartialBlock {
        PartialBlock {
            header: None,
            transactions: TransactionContainer::new(),
        }
   }

    pub fn calc_header(&mut self, prev_hash: Digest) {

        let merkle_root = self.transactions.merkle_root();
        let target = &TARGET;

        self.header = Some(PartialBlockHeader::new(prev_hash, target, merkle_root));
    }
}
