use bytes::Bytes;

pub struct OutPoint {
    /// Transaction id of the transaction holding the utxo
    pub hash: String,
    /// The number of the output index number
    pub index: u32,
}
pub struct TransactionInput {
    pub previous_output: OutPoint,
    pub script_sig: Bytes,
}
pub struct TransactionOutput {
    pub value: u64,
    pub script_pubkey: Bytes,
}
