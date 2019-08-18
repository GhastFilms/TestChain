use ring::digest::{Digest, Context};

#[derive(Clone)]
pub struct Input {
    from_hash: Digest,
    from_index: u32,
}

impl Input {
    pub fn hash(&self, hasher: &mut Context) {
        hasher.update(self.from_hash.as_ref());
        hasher.update(&self.from_index.to_le_bytes());
    }
}

//TODO a previous block should be passed into the input builder and a transaction should be chosen from the list of transactions

pub struct InputBuilder {
    from_hash: Option<Digest>,
    from_index: Option<u32>,
}    
 
impl InputBuilder {
    pub fn new() -> InputBuilder {
        InputBuilder {
            from_hash: None,
            from_index: None,
        }
    }
}
