use base58::FromBase58;
use secp256k1::Error;
use std::str::FromStr;
use AddressHash;

pub struct Address {
    pub hash: AddressHash,
}

impl FromStr for Address {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        let hex = try!(s.from_base58().map_err(|_| Error::InvalidSecretKey));

        if hex.len() != 25 {
            return Err(Error::InvalidSecretKey);
        }

        let hash = AddressHash::from_str(s);

        match hash {
            Ok(v) => {
                return Ok(Address { hash: v });
            }
            Err(_) => return Err(Error::InvalidPublicKey),
        }
    }
}
