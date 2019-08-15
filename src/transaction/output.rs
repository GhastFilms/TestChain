use ring::digest::{Digest, Context};
use ring::signature::{KeyPair, EcdsaKeyPair};


#[derive(Clone)]
pub struct TxOutput {
    value: u64,
    to: <EcdsaKeyPair as KeyPair>::PublicKey,
}

impl TxOutput {
    pub fn hash(&self, hasher: &mut Context) {
        hasher.update(self.to.as_ref());
        hasher.update(&self.value.to_le_bytes());
    }
}

