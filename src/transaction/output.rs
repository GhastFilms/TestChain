use ring::digest::Context;
use ring::signature::{KeyPair, EcdsaKeyPair};

#[derive(Clone)]
pub struct Output {
    value: u64,
    to: <EcdsaKeyPair as KeyPair>::PublicKey,
}

impl Output {
    pub fn hash(&self, hasher: &mut Context) {
        hasher.update(self.to.as_ref());
        hasher.update(&self.value.to_le_bytes());
    }
}
