use base58::FromBase58;
use secp256k1::Error;
use std::str::FromStr;
use {Message, Secret, Signature};

#[derive(PartialEq)]
pub struct Private {
    /// ECDSA key.
    pub secret: Secret,
}

impl Private {
    /// Function that signs a message and returns the signature
    pub fn sign(&self, _message: &Message) -> Signature {
        unimplemented!();
    }

    pub fn from_secret(secret: Secret) -> Self {
        Private { secret: secret }
    }
}

impl FromStr for Private {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        let hex = try!(s.from_base58().map_err(|_| Error::InvalidSecretKey));

        if hex.len() != 33 {
            return Err(Error::InvalidSecretKey);
        }

        let secret = Secret::from_str(s);

        match secret {
            Ok(v) => {
                return Ok(Private { secret: v });
            }
            Err(_) => return Err(Error::InvalidSecretKey),
        }
    }
}
