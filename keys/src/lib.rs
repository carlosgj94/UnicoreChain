extern crate base58;
#[macro_use]
extern crate lazy_static;
extern crate secp256k1;
extern crate unicore_primitives as primitives;

use hash::{H160, H256, H520};
pub use primitives::{bytes, hash};

mod address;
mod keypair;
mod private;
mod public;

pub use address::Address;
pub use keypair::KeyPair;
pub use private::Private;
pub use public::Public;

/// 32 bytes long secret key
pub type Secret = H256;
/// 32 bytes long signable message
pub type Message = H256;
/// 32 bytes long Signature
pub type Signature = H520;
/// 20 bytes long hash derived from public `ripemd160(sha256(public))`
pub type AddressHash = H160;

lazy_static! {
    pub static ref SECP256K1: secp256k1::Secp256k1 = secp256k1::Secp256k1::new();
}
