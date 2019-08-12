use ring::digest::{digest, Context, Digest, SHA256};
use ring::error::Unspecified;
use ring::signature::{EcdsaKeyPair, KeyPair, Signature, UnparsedPublicKey, ED25519};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

//to save on network usage transactions are ordered but least to greatest by their hash value
//the TransactionContainer enforces that to make sure that everything gets hashed and stored in the right order.

/// TransactionContainer manages the hashing and verifying of transactions.
/// It also manages the order
pub struct TransactionContainer {
    transactions: HashMap<Digest, Transaction>,
    transactions_hashes: Vec<Digest>,
    transaction_count: u32,
}

impl TransactionContainer {

    // transactions cant be removed, they are eternal.
    fn insert(&mut self, tx: Transaction) {
        //ignore if its a duplicate
        //add to hashlist and hashmap
    }

    fn add_bulk(&mut self, txs: Vec<Transaction>) {
        //check for duplicates,
        //sort
        //insert values into hashlist
        //insert values into
        //add to transaction count
    }

    pub fn check_order(&mut self) -> bool {
        // the default value is 0 because rustc complains about how prev might not be initialized, it will be over ridden when it starts
        let mut prev: &[u8] = &[0];
        for (i, x) in self.transactions_hashes.iter().enumerate() {
            if i == 0 {
                prev = x.as_ref();
                break;
            }

            if x.as_ref() < prev {
                return false;
            }

            prev = x.as_ref();
        }
        true
    }

    pub fn merkle_root(&self) -> Result<Digest, ()> {
        let i = &self.transactions_hashes;
        
        if i.is_empty() {
            println!("no values provided");
            Err(())
        } else {
            let d: Vec<&Digest> = i.iter().collect();
            Ok(TransactionContainer::_merkle_root(d))
        }
    }

    fn _merkle_root(mut i: Vec<&Digest>) -> Digest {
        if i.len() == 1 {
            **i.get(0).unwrap()
        } else {
            let snd: Vec<&Digest> = i.split_off((i.len() as f64 / 2.0).floor() as usize);
            let mut x = TransactionContainer::_merkle_root(i).as_ref().to_vec();
            x.extend_from_slice(TransactionContainer::_merkle_root(snd).as_ref());
            digest(&SHA256, &x)
        }
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
    /// Gets the full hash of the transaction including signature.
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

    pub fn verify(&self, key: <EcdsaKeyPair as KeyPair>::PublicKey) -> Result<(), Unspecified> {
        let mut hasher = Context::new(&SHA256);
        self.no_sig_hash(&mut hasher);
        let digest = hasher.finish();

        let pk = UnparsedPublicKey::new(&ED25519, key.as_ref());

        pk.verify(digest.as_ref(), self.sig.as_ref())
    }

    /// Gets the
    pub fn no_sig_hash(&self, hasher: &mut Context) {
        hasher.update(self.tx_id.as_ref());
        hasher.update(&self.version.to_le_bytes());
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
