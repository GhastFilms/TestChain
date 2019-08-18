use ring::digest::{Digest, SHA256, Context};
use ring::signature::{Signature, KeyPair, EcdsaKeyPair, UnparsedPublicKey, ED25519};
use ring::error::Unspecified;
use ring::rand;

use super::{Input, Output};

#[derive(Clone)]
pub struct Transaction {
    pub version: u32,
    pub sig: Signature,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
    pub lock_time: i64,
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


/// TransactionBuilder is used to build transactions
/// 
/// All inputs and outputs entered should be valid, transactions with double spent inputs will be rejected.
///
pub struct TransactionBuilder {
    version: u32,
    inputs: Vec<Input>,
    outputs: Vec<Output>,
    lock_time: Option<i64>,
}

fn input_compare(fst: &Input, snd: &Input) -> bool {
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


fn output_compare(fst: &Output, snd: &Output) -> bool {
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
    pub fn push_input(&mut self, i: Input) {
        // ignores duplicate inputs
        for x in &self.inputs {
            if input_compare(&i, &x) {
                return;
            }
        }
        self.inputs.push(i);
    }

    pub fn get_inputs(&self) -> Vec<Input> {
        self.inputs.clone()
    }

    pub fn pop_input(&mut self) {
        self.inputs.pop();
    }
}

//output handling functions
impl TransactionBuilder {
    pub fn push_output(&mut self, i: Output) {
        for x in &self.outputs {
            if output_compare(&i, &x) {
                return;
            }
        }
        self.outputs.push(i);
    }

    pub fn get_outputs(&self) -> Vec<Output> {
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


    pub fn sort_inputs(i: Vec<Input>) -> Vec<Input> {
        let v:  Vec<(Digest, Input)> = i.into_iter().map(|x| {
            let mut h = Context::new(&SHA256);
            x.hash(&mut h);
            let hash = h.finish();
            
            (hash, x)
        }).collect();
        TransactionBuilder::sort(v).into_iter().map(|x| {
            x.1
        }).collect()
    }

    pub fn sort_outputs(i: Vec<Output>) -> Vec<Output> {
        let v:  Vec<(Digest, Output)> = i.into_iter().map(|x| {
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
