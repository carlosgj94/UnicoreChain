use rand::os::OsRng;
use secp256k1::Error;
use {KeyPair, Private, SECP256K1, Secret};

pub trait Generator {
    fn generate(&self) -> Result<KeyPair, Error>;
}

impl Generator {
    pub fn generate(&self) -> Result<KeyPair, Error> {
        let context = &SECP256K1;
        let mut rng = try!(OsRng::new().map_err(|_| Error::InvalidSecretKey));
        let (secret, _public) = try!(context.generate_keypair(&mut rng));
        let mut secret_hash = Secret::default();
        secret_hash.copy_from_slice(&secret[0..32]);
        KeyPair::from_private(Private::from_secret(secret_hash))
    }
}
