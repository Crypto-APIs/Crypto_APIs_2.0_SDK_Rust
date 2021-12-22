/*
 * CryptoAPIs
 *
 * Crypto APIs 2.0 is a complex and innovative infrastructure layer that radically simplifies the development of any Blockchain and Crypto related applications. Organized around REST, Crypto APIs 2.0 can assist both novice Bitcoin/Ethereum enthusiasts and crypto experts with the development of their blockchain applications. Crypto APIs 2.0 provides unified endpoints and data, raw data, automatic tokens and coins forwardings, callback functionalities, and much more.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: developers@cryptoapis.io
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetTransactionDetailsByTransactionIdFromCallbackRi {
    /// Represents the index position of the transaction in the specific block.
    #[serde(rename = "index")]
    pub index: i32,
    /// Represents the hash of the block where this transaction was mined/confirmed for first time. The hash is defined as a cryptographic digital fingerprint made by hashing the block header twice through the SHA256 algorithm.
    #[serde(rename = "minedInBlockHash")]
    pub mined_in_block_hash: String,
    /// Represents the hight of the block where this transaction was mined/confirmed for first time. The height is defined as the number of blocks in the blockchain preceding this specific block.
    #[serde(rename = "minedInBlockHeight")]
    pub mined_in_block_height: i32,
    /// Represents a list of recipient addresses with the respective amounts. In account-based protocols like Ethereum there is only one address in this list.
    #[serde(rename = "recipients")]
    pub recipients: Vec<crate::models::GetTransactionDetailsByTransactionIdFromCallbackRiRecipients>,
    /// Represents a list of sender addresses with the respective amounts. In account-based protocols like Ethereum there is only one address in this list.
    #[serde(rename = "senders")]
    pub senders: Vec<crate::models::GetTransactionDetailsByTransactionIdFromCallbackRiSenders>,
    /// Defines the exact date/time in Unix Timestamp when this transaction was mined, confirmed or first seen in Mempool, if it is unconfirmed.
    #[serde(rename = "timestamp")]
    pub timestamp: i32,
    /// Represents the same as transactionId for account-based protocols like Ethereum, while it could be different in UTXO-based protocols like Bitcoin. E.g., in UTXO-based protocols hash is different from transactionId for SegWit transactions.
    #[serde(rename = "transactionHash")]
    pub transaction_hash: String,
    #[serde(rename = "fee")]
    pub fee: Box<crate::models::GetTransactionDetailsByTransactionIdFromCallbackRiFee>,
    #[serde(rename = "isConfirmed")]
    pub is_confirmed: bool,
    #[serde(rename = "blockchainSpecific")]
    pub blockchain_specific: Box<crate::models::GetTransactionDetailsByTransactionIdFromCallbackRibs>,
}

impl GetTransactionDetailsByTransactionIdFromCallbackRi {
    pub fn new(index: i32, mined_in_block_hash: String, mined_in_block_height: i32, recipients: Vec<crate::models::GetTransactionDetailsByTransactionIdFromCallbackRiRecipients>, senders: Vec<crate::models::GetTransactionDetailsByTransactionIdFromCallbackRiSenders>, timestamp: i32, transaction_hash: String, fee: crate::models::GetTransactionDetailsByTransactionIdFromCallbackRiFee, is_confirmed: bool, blockchain_specific: crate::models::GetTransactionDetailsByTransactionIdFromCallbackRibs) -> GetTransactionDetailsByTransactionIdFromCallbackRi {
        GetTransactionDetailsByTransactionIdFromCallbackRi {
            index,
            mined_in_block_hash,
            mined_in_block_height,
            recipients,
            senders,
            timestamp,
            transaction_hash,
            fee: Box::new(fee),
            is_confirmed,
            blockchain_specific: Box::new(blockchain_specific),
        }
    }
}


