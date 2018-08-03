use {BlockHeader, Transaction};
pub struct Block {
    /// The hash of the block header
    pub block_header: BlockHeader,
    pub transactions: Vec<Transaction>,
}

impl Block {}
