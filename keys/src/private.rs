use Secret;

#[derive(PartialEq)]
pub struct Private {
    /// The network where the key is being used
    pub network: u32,
    /// ECDSA key.
    pub secret: Secret,
}
