pub struct BlockHeader {
    /// Previous block hash
    pub p_header_hash: String,
    /// The hash of the merkle root that contains the transactions
    pub merkle_root_hash: String,
    /// The time is starting to be mined
    pub time: u32,
    /// The magical number that has to be found
    pub nonce: String,
    /// Difficulty of the block
    pub bits: u8,
}
