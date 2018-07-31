use rand::os::OsRng;
use secp256k1::Error;
use secp256k1::Secp256k1;
use {KeyPair, Private, Public};

pub trait Generator {
    fn generate(&self) -> KeyPair;
}

impl Generator {
    pub fn generate(&self) -> KeyPair {
        let context = Secp256k1::new();
        let rng = OsRng::new().map_err(|_| Error::InvalidSecretKey).ok();
        let res = context.generate_keypair(&mut rng.unwrap());
        let (secret, publickey) = res.unwrap();
        let private = Private::from_secret_key(secret);
        let public = Public::from_public_key(publickey);
        KeyPair::from_keypair(private, public)
    }
}
