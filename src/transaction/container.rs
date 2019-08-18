use super::Transaction;

use std::collections::HashMap;
use ring::digest::{Digest, digest, SHA256, Context};
use ring::signature::{KeyPair, EcdsaKeyPair};
//to save on network usage transactions are ordered but least to greatest by their hash value
//the TransactionContainer enforces that to make sure that everything gets hashed and stored in the right order.

use super::{TransactionBuilder, Output};

/// TransactionContainer manages the hashing and verifying of transactions.
/// 
/// It also manages the order of the transactions
/// Transactions are ordered from least to greatest based on their hash value
pub struct TransactionContainer {
    transactions: Vec<Transaction>,
    transaction_count: u32,
}

const DEFAULT_TX_MR: &'static str = "RDewALDMxtpWkFuYZ5ePhw4tKYFSWw8UFP5czcJ9E9W3GPZjQqb55Nybt33uMAz8";

impl TransactionContainer {
    pub fn new() -> TransactionContainer {
        TransactionContainer {
            transactions: Vec::new(),
            transaction_count: 0,
        }
    }

    pub fn get_transactions(&self) -> Vec<&Transaction> {
        self.transactions.iter().collect()   
    }

    // transactions cant be removed, they are eternal.
    fn insert(&mut self, tx: Transaction) {
        // this is kidna dumb but i guess it works
        if ! &self.dupe_check(&tx)  {
            &self.transactions.push(tx);
            self.transactions = TransactionContainer::sort(self.transactions.clone());
            self.transaction_count += 1;
        }
    }

    fn insert_bulk(&mut self, txs: Vec<Transaction>) {
        for x in txs {
            &self.insert(x);
        }
    }

    //just resort them :/ or if the order needs to be checked just do it manually with
    //get_transactions
    /*
    pub fn check_order(&mut self) -> bool {
        // the default value is 0 because rustc complains about how prev might not be initialized, it will be over ridden when it starts
        let mut prev: &[u8] = &[0];
        for (i, x) in self.transactions.iter().enumerate() {
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
    }*/
    
    pub fn merkle_root(&self) -> Digest {
        let i = &self.transactions;

        if i.is_empty() {
            
            digest(&SHA256, DEFAULT_TX_MR.as_bytes() )
        } else {
            let d: Vec<Digest> = i.iter().map(|x| {
                let mut h = Context::new(&SHA256);
                x.hash(&mut h);
                h.finish()
            }).collect();
            TransactionContainer::_merkle_root(d)
        }
    }

    fn _merkle_root(mut i: Vec<Digest>) -> Digest {
        
        if i.len() == 1 {
            *i.get(0).unwrap()
        } else {
            let snd: Vec<Digest> = i.split_off((i.len() as f64 / 2.0).floor() as usize);
            let mut x = TransactionContainer::_merkle_root(i).as_ref().to_vec();
            x.extend_from_slice(TransactionContainer::_merkle_root(snd).as_ref());
            digest(&SHA256, &x)
        }
    }
}

impl TransactionContainer {
    fn sorter<T: Clone>(i: Vec<(Digest, T)>) -> Vec<(Digest, T)> {
        
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


    fn sort(i: Vec<Transaction>) -> Vec<Transaction> {
        let v:  Vec<(Digest, Transaction)> = i.into_iter().map(|x| {
            let mut h = Context::new(&SHA256);
            x.hash(&mut h);
            let hash = h.finish();
            
            (hash, x)
        }).collect();
        TransactionContainer::sorter(v).into_iter().map(|x| {
            x.1
        }).collect()
    }

    fn dupe_check(&self, t: &Transaction) -> bool {
        let t_h = {
            let mut h = Context::new(&SHA256);
            t.hash(&mut h);
            h.finish()
        };

        self.transactions.iter().find(move | x | {
            let mut h = Context::new(&SHA256);
            x.hash(&mut h);
            h.finish().as_ref() == t_h.as_ref()
        }).is_some()
        
    }
}
