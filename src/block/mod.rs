mod block;
mod header;

pub use header::*;
pub use block::*;

use ring::{
    digest::Digest, digest::Context, digest::SHA256
};

const VERSION: u32 = 1;

const TARGET: [u8; 32] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
