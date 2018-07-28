extern crate base58;
extern crate secp256k1;
extern crate unicore_primitives as primitives;

use hash::{H160, H256};
pub use primitives::{bytes, hash};

pub mod private;

/// 32 bytes long secret key
pub type Secret = H256;
