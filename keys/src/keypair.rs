use hash::H264;
use secp256k1::{key, Error};
use {Address, Private, Public, SECP256K1};

pub struct KeyPair {
    private: Private,
    public: Public,
}

impl KeyPair {
    pub fn private(&self) -> &Private {
        &self.private
    }
    pub fn public(&self) -> &Public {
        &self.public
    }
    pub fn address(&self) -> &Address {
        unimplemented!();
    }

    pub fn from_private(private: Private) -> Result<Self, Error> {
        let context = &SECP256K1;
        let s: key::SecretKey = try!(key::SecretKey::from_slice(context, &*private.secret));

        let pub_key = try!(key::PublicKey::from_secret_key(context, &s));
        let serialized = pub_key.serialize_vec(context, true);

        let public = {
            let mut public = H264::default();
            public.copy_from_slice(&serialized[0..33]);
            Public { address: public }
        };

        Ok(KeyPair {
            private: private,
            public: public,
        })
    }
}
