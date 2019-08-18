use super::Transaction;

use std::collections::HashMap;
use ring::digest::{Digest, digest, SHA256, Context};
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
