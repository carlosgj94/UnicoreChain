use bytes::Bytes;
use primitives::H256;

pub struct OutPoint {
    /// Transaction id of the transaction holding the utxo
    pub hash: H256,
    /// The number of the output index number
    pub index: u32,
}
pub struct TransactionInput {
    /// The transaction from where this transaction comes from
    pub previous_output: OutPoint,
    /// The signature
    pub script_sig: Bytes,
    /// The number of that transaction in the output
    pub vout: u32,
}
pub struct TransactionOutput {
    pub value: u64,
    pub script_pubkey: Bytes,
}
