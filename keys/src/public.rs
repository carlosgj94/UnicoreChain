use base58::FromBase58;
use crypto::dhash160;
use hash::H264;
use secp256k1::Error;
use std::str::FromStr;
use {AddressHash, Message, Signature};

#[derive(PartialEq)]
pub struct Public {
    /// Compressed version of the public key
    pub address: H264,
}

impl Public {
    pub fn verify(&self, _message: &Message, _signature: &Signature) -> Result<bool, Error> {
        unimplemented!();
    }

    fn from_slice(data: &[u8]) -> Result<Self, Error> {
        if data.len() == 33 {
            let mut public = H264::default();
            public.copy_from_slice(data);
            Ok(Public { address: public })
        } else {
            Err(Error::InvalidPublicKey)
        }
    }

    pub fn address_hash(&self) -> AddressHash {
        dhash160(&self.address.clone().take())
    }
}

impl FromStr for Public {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        let hex = try!(s.from_base58().map_err(|_| Error::InvalidSecretKey));
        if hex.len() != 33 {
            return Err(Error::InvalidSecretKey);
        }

        let public = H264::from_str(s);
        match public {
            Ok(v) => {
                return Ok(Public { address: v });
            }
            Err(_) => return Err(Error::InvalidPublicKey),
        }
    }
}
