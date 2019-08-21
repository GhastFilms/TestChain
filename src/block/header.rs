
use ring::{
    digest::Digest, digest::Context, digest::SHA256
};


use super::block::*;
use super::VERSION;
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




pub struct PartialBlockHeader {
    pub version: u32,
    pub prev_hash: Digest,
    pub target: Vec<u8>,
    pub merkle_root: Digest,
    pub time: u64,
    pub nonce: Option<u64>,
}


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
    /// Checks to make sure the current content of the PartialBlockHeader is valid for a full BlockHeader
    /// Returns Error if any of the tests fail
    pub fn to_full_header(&self) -> Result<(), Error> {      
        Ok(())       
    }
}



pub enum Error {

}
