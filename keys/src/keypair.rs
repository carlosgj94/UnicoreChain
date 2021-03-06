use {Private, Public};

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
    //pub fn address(&self) -> &Address {
    //    unimplemented!();
    //}

    pub fn from_keypair(private: Private, public: Public) -> Self {
        KeyPair {
            private: private,
            public: public,
        }
    }
}
