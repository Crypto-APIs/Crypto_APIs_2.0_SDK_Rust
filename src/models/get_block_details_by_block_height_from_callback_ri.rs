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
pub struct GetBlockDetailsByBlockHeightFromCallbackRi {
    /// Represents the hash of the block, which is its unique identifier. It represents a cryptographic digital fingerprint made by hashing the block header twice through the SHA256 algorithm.
    #[serde(rename = "hash")]
    pub hash: String,
    /// Represents the number of blocks in the blockchain preceding this specific block. Block numbers have no gaps. A blockchain usually starts with block 0 called the \"Genesis block\".
    #[serde(rename = "height")]
    pub height: i32,
    /// Represents the hash of the previous block, also known as the parent block.
    #[serde(rename = "previousBlockHash")]
    pub previous_block_hash: String,
    /// Defines the exact date/time when this block was mined in Unix Timestamp.
    #[serde(rename = "timestamp")]
    pub timestamp: i32,
    /// Represents the total number of all transactions as part of this block.
    #[serde(rename = "transactionsCount")]
    pub transactions_count: i32,
    #[serde(rename = "blockchainSpecific")]
    pub blockchain_specific: Box<crate::models::GetBlockDetailsByBlockHeightFromCallbackRibs>,
}

impl GetBlockDetailsByBlockHeightFromCallbackRi {
    pub fn new(hash: String, height: i32, previous_block_hash: String, timestamp: i32, transactions_count: i32, blockchain_specific: crate::models::GetBlockDetailsByBlockHeightFromCallbackRibs) -> GetBlockDetailsByBlockHeightFromCallbackRi {
        GetBlockDetailsByBlockHeightFromCallbackRi {
            hash,
            height,
            previous_block_hash,
            timestamp,
            transactions_count,
            blockchain_specific: Box::new(blockchain_specific),
        }
    }
}


