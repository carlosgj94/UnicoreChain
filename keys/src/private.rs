use base58::ToBase58;
pub use rustc_serialize::hex::ToHex;
use secp256k1::key::SecretKey;
use std::fmt;

#[derive(PartialEq)]
pub struct Private(SecretKey);

impl Private {
    /// Function that signs a message and returns the signature
    pub fn sign(&self) {
        unimplemented!();
    }
    pub fn from_secret_key(secret: SecretKey) -> Self {
        Private(secret)
    }
}

impl fmt::Display for Private {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let mut content = vec![];
        content.extend(self.0[0..].iter());
        fmt.write_str(&content.to_base58())?;
        Ok(())
    }
}
