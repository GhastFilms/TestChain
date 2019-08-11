extern crate bincode;
extern crate ring;
extern crate serde;
#[macro_use] extern crate serde_derive;

use ring::digest::{digest, Digest, SHA256};
use ring::signature::{
    KeyPair,
    EcdsaKeyPair
};
use std::{boxed::Box, io::Read};
use std::rc::Rc;
use serde::{Deserialize, Serialize};


pub struct Block {
    header: BlockHeader,
    transactions: TransactionContainer,
    Size: u32,
}

pub struct BlockHeader {
    version: u32,
    prev_hash: Digest,
    target: u32,
    merkle_root: Digest,
    time: u32,
    nonce: Option<u64>,
}


pub struct TransactionContainer {
    transactions: Vec<Transaction>,
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
    sig: ring::signature::Signature, 
    inputs: Vec<TxInput>,
    outputs: Vec<TxOutput>,
    lock_time: u32,
}

struct TxInput {
    from_hash: Digest,
    from_index: u32,
}

struct TxOutput {
    value: u64,
    to: <ring::signature::EcdsaKeyPair as ring::signature::KeyPair>::PublicKey, 
}

//TODO merkle root functions should be converted to a method of TransactionContainer
//TODO remove bincode

pub fn merkle_root<T: Serialize>(i: &Vec<T>) -> Result<Digest, ()> {
    if i.is_empty() {
        println!("no values provided");
        Err(())
    } else {
        let d: Vec<&T> = i.iter().collect();
        Ok(_merkle_root(d))
    }
}

fn _merkle_root<T: Serialize>(mut i: Vec<&T>) -> Digest {
    if i.len() == 1 {
        calculate_hash(&i.get(0))
    } else {
        let snd: Vec<&T> = i.split_off((i.len() as f64 / 2.0).floor() as usize);
        let mut x = _merkle_root(i).as_ref().to_vec();
        x.extend_from_slice(_merkle_root(snd).as_ref());
        calculate_hash(&x)
    }
}

fn calculate_hash<T: Serialize>(t: &T) -> Digest {
    let bytes = any_as_u8_slice(t);
    digest(&SHA256, &bytes)
}

fn any_as_u8_slice<T: Sized + Serialize>(p: &T) -> Vec<u8> {
    bincode::serialize(&p).unwrap()
}

mod merkle_test {
    use super::merkle_root;
    use ring::digest::Digest;
    #[test]
    fn merkle_root_test() {
        let x = String::from("8710D98CBA52069F0115C5ED68782BD59B68A97E0FB261A418DE12F4A27E1F6");
        let data = vec![
            String::from("a"),
            String::from("b"),
            String::from("c"),
            String::from("d"),
        ];
        assert_eq!(x, digest_as_hex(&merkle_root(&data).unwrap()));
    }

    #[test]
    fn merkle_root_odd() {
        let x = String::from("EF3344F647EE2557A5E995D7A6C762D3A63B828278FF9B3656EC20D82B9728");
        let data = vec![
            String::from("a"),
            String::from("b"),
            String::from("c"),
            String::from("d"),
            String::from("e"),
        ];
        assert_eq!(x, digest_as_hex(&merkle_root(&data).unwrap()));
    }

    fn digest_as_hex(i: &Digest) -> String {
        i.as_ref().iter().fold(String::new(), |mut acc, x| {
            acc.push_str(&format!("{:X}", x));
            acc
        })
    }
}
