extern crate base58;
extern crate secp256k1;
extern crate unicore_primitives as primitives;

use hash::{H256, H520};
pub use primitives::{bytes, hash};

pub mod private;

/// 32 bytes long secret key
pub type Secret = H256;
/// 32 bytes long signable message
pub type Message = H256;
/// 32 bytes long Signature
pub type Signature = H520;
