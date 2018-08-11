use primitives::H256;

pub struct BlockHeader {
    /// Previous block hash
    pub p_header_hash: H256,
    /// The hash of the merkle root that contains the transactions
    pub merkle_root_hash: H256,
    /// The time is starting to be mined
    pub time: u32,
    /// The magical number that has to be found
    pub nonce: u64,
    /// Difficulty of the block
    pub bits: u8,
}
