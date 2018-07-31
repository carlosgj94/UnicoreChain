use base58::ToBase58;
pub use rustc_serialize::hex::ToHex;
use secp256k1::key::PublicKey;
use secp256k1::Error;
use secp256k1::Secp256k1;
use std::fmt;

#[derive(PartialEq)]
pub struct Public(PublicKey);

impl Public {
    pub fn verify(&self) -> Result<bool, Error> {
        unimplemented!();
    }

    pub fn from_public_key(public: PublicKey) -> Self {
        Public(public)
    }
}

impl fmt::Display for Public {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let context = Secp256k1::new();
        let content = &self.0.serialize_vec(&context, false).as_slice().to_base58();
        fmt.write_str(content)?;
        Ok(())
    }
}
