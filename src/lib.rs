#![allow(dead_code)]
extern crate ring;

use ring::{
    digest::Digest, digest::Context, digest::SHA256
};

pub mod transaction;
pub mod block;

use block::{ Block, BlockHeader};
use transaction::TransactionContainer;
