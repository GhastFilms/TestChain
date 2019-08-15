use ring::digest::{Digest, Context};





#[derive(Clone)]
pub struct TxInput {
    from_hash: Digest,
    from_index: u32,
}

impl TxInput {
    pub fn hash(&self, hasher: &mut Context) {
        hasher.update(self.from_hash.as_ref());
        hasher.update(&self.from_index.to_le_bytes());
    }
}
