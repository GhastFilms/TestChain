use ring::digest::{digest, Context, Digest, SHA256};
use ring::rand;
use ring::error::Unspecified;
use ring::signature::{EcdsaKeyPair, KeyPair, Signature, UnparsedPublicKey, ED25519};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

//to save on network usage transactions are ordered but least to greatest by their hash value
//the TransactionContainer enforces that to make sure that everything gets hashed and stored in the right order.

/// TransactionContainer manages the hashing and verifying of transactions.
/// 
/// It also manages the order of the transactions
/// Transactions are ordered from least to greatest based on their hash value
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
    version: u32,
    sig: Signature,
    inputs: Vec<TxInput>,
    outputs: Vec<TxOutput>,
    lock_time: i64,
}

impl Transaction {
    /// Gets the full hash of the transaction including signature.
    pub fn hash(&self, hasher: &mut Context) {
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

    /// Gets the hash
    pub fn no_sig_hash(&self, hasher: &mut Context) {
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





/// TransactionBuilder is used to build transactions
/// 
/// All inputs and outputs entered should be valid, transactions with double spent inputs will be rejected.
///
pub struct TransactionBuilder {
    version: u32,
    inputs: Vec<TxInput>,
    outputs: Vec<TxOutput>,
    lock_time: Option<i64>,
}

fn input_compare(fst: &TxInput, snd: &TxInput) -> bool {
    let fst_hash = {
        let mut h = Context::new(&SHA256);
        fst.hash(&mut h);
        h.finish()
    };
            
    let snd_hash = {
        let mut h = Context::new(&SHA256);
        snd.hash(&mut h);
        h.finish()
    };

    fst_hash.as_ref() == snd_hash.as_ref()
}


fn output_compare(fst: &TxOutput, snd: &TxOutput) -> bool {
    let fst_hash = {
        let mut h = Context::new(&SHA256);
        fst.hash(&mut h);
        h.finish()
    };
            
    let snd_hash = {
        let mut h = Context::new(&SHA256);
        snd.hash(&mut h);
        h.finish()
    };

    fst_hash.as_ref() == snd_hash.as_ref()
}



// inputs handling functions
impl TransactionBuilder {
    pub fn push_input(&mut self, i: TxInput) {
        // ignores duplicate inputs
        for x in &self.inputs {
            if input_compare(&i, &x) {
                return;
            }
        }
        self.inputs.push(i);
    }

    pub fn get_inputs(&self) -> Vec<TxInput> {
        self.inputs.clone()
    }

    pub fn pop_input(&mut self) {
        self.inputs.pop();
    }
}

//output handling functions
impl TransactionBuilder {
    pub fn push_output(&mut self, i: TxOutput) {
        for x in &self.outputs {
            if output_compare(&i, &x) {
                return;
            }
        }
        self.outputs.push(i);
    }

    pub fn get_outputs(&self) -> Vec<TxOutput> {
        self.outputs.clone()
    }

    pub fn pop_output(&mut self) {
        self.outputs.pop();
    }
}


// sorting functions
impl TransactionBuilder {
    /// sorts inputs and outputs
    ///
    /// currently uses bubble sort but may use another algorithm in the future
    fn sort<T: Clone>(i: Vec<(Digest, T)>) -> Vec<(Digest, T)> {
        
        let mut input = i.clone();

        let n = input.len();
        for i in 0..n {
            let mut swap_occured: bool = false;
            for j in 0..n-i-1 {
                if input[j].0.as_ref() > input[j+1].0.as_ref() {
                    input.swap(j,j+1);
                    swap_occured = true;
                }
            }
            if !swap_occured {
                break;
            }
        }
        
        input.to_vec()
    }


    pub fn sort_inputs(i: Vec<TxInput>) -> Vec<TxInput> {
        let v:  Vec<(Digest, TxInput)> = i.into_iter().map(|x| {
            let mut h = Context::new(&SHA256);
            x.hash(&mut h);
            let hash = h.finish();
            
            (hash, x)
        }).collect();
        TransactionBuilder::sort(v).into_iter().map(|x| {
            x.1
        }).collect()
    }

    pub fn sort_outputs(i: Vec<TxOutput>) -> Vec<TxOutput> {
        let v:  Vec<(Digest, TxOutput)> = i.into_iter().map(|x| {
            let mut h = Context::new(&SHA256);
            x.hash(&mut h);
            let hash = h.finish();

            (hash, x)
        }).collect();
        TransactionBuilder::sort(v).into_iter().map(|x| {
            x.1
        }).collect()
    }
}


// starting and signing
impl TransactionBuilder {
    pub fn new() -> TransactionBuilder {
        TransactionBuilder {
            version: 1,
            inputs: Vec::new(),
            outputs: Vec::new(),
            lock_time: None,
        }
    }

    pub fn sign(self, k: EcdsaKeyPair) -> Result<Transaction, Error> {
        
        let version = self.version;
        let lock_time = chrono::Local::now().timestamp();

        // these need to be checked and sorted
        // input checks should verify if the signing key is valid for the inputs
        let inputs = TransactionBuilder::sort_inputs(self.inputs);
        let outputs = TransactionBuilder::sort_outputs(self.outputs);


        // if any errors other than signing error occur they should have happened before this hash is created
        let hash = {
            let mut hasher = Context::new(&SHA256);
            hasher.update(&version.to_le_bytes());

            for x in inputs.iter() {
                &x.hash(&mut hasher);
            }

            for x in outputs.iter() {
                &x.hash(&mut hasher);
            }

            hasher.update(&lock_time.to_le_bytes());
            
            hasher.finish()
        };

        let rng = rand::SystemRandom::new();
        let sig = match k.sign(&rng, hash.as_ref()) {
            Ok(s) => {
                s
            },
            Err(e) => {
                return Err(Error::SigningError(e));
            },
        };

        let r = Transaction {
            version,
            sig,
            inputs,
            outputs,
            lock_time,
        };
        Ok(r)
    }
}

pub enum Error {
    MissingInputs,
    MissingOutpus,
    Unsigned,
    InvalidKeyPair,
    SigningError(Unspecified),
}
