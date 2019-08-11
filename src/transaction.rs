use ring::digest::{digest, Digest, SHA256, Context};
use ring::signature::{EcdsaKeyPair, KeyPair, Signature};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

//to save on network usage transactions are ordered but least to greatest by their hash value
//the TransactionContainer enforces that to make sure that everything gets hashed and stored in the right order.
pub struct TransactionContainer {
    transactions: HashMap<Digest, Transaction>,
    //transactions_hashes: Vec<Digest>,
    transaction_count: u32,
}

impl TransactionContainer {
    // transactions cant be removed, they are eternal.
    pub fn add_transaction(&mut self, tx: Transaction) -> Result<(), ()> {
        Err(()) // not implimented yet
    }
}

pub struct Transaction {
    tx_id: Digest,
    version: u32,
    sig: Signature,
    inputs: Vec<TxInput>,
    outputs: Vec<TxOutput>,
    lock_time: u32,
}

impl Transaction {
    pub fn hash(&self, hasher: &mut Context) {
        hasher.update(self.tx_id.as_ref());
        hasher.update(&self.version.to_le_bytes());
        hasher.update(self.sig.as_ref());
        
        for x in self.inputs.iter() {
            x.hash(hasher);
        }
        
        for x in self.outputs.iter() {
            x.hash(hasher);
        }

        hasher.update(&self.lock_time.to_le_bytes());
    }
}

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