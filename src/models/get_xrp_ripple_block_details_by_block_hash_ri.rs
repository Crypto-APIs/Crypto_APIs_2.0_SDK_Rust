/*
 * CryptoAPIs
 *
 * Crypto APIs 2.0 is a complex and innovative infrastructure layer that radically simplifies the development of any Blockchain and Crypto related applications. Organized around REST, Crypto APIs 2.0 can assist both novice Bitcoin/Ethereum enthusiasts and crypto experts with the development of their blockchain applications. Crypto APIs 2.0 provides unified endpoints and data, raw data, automatic tokens and coins forwardings, callback functionalities, and much more.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: developers@cryptoapis.io
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct GetXrpRippleBlockDetailsByBlockHashRi {
    /// Represents the hash of the block, which is its unique identifier. It represents a cryptographic digital fingerprint made by hashing the block header twice through the SHA256 algorithm.
    #[serde(rename = "blockHash")]
    pub block_hash: String,
    /// Represents the number of blocks in the blockchain preceding this specific block. Block numbers have no gaps. A blockchain usually starts with block 0 called the \"Genesis block\".
    #[serde(rename = "blockHeight")]
    pub block_height: i32,
    /// Represents the hash of the next block. When this is the last block of the blockchain this value will be an empty string.
    #[serde(rename = "nextBlockHash")]
    pub next_block_hash: String,
    /// Represents the hash of the previous block, also known as the parent block.
    #[serde(rename = "previousBlockHash")]
    pub previous_block_hash: String,
    /// Defines the exact date/time when this block was mined in Unix Timestamp.
    #[serde(rename = "timestamp")]
    pub timestamp: i32,
    #[serde(rename = "totalCoins")]
    pub total_coins: Box<crate::models::GetXrpRippleBlockDetailsByBlockHashRiTotalCoins>,
    #[serde(rename = "totalFees")]
    pub total_fees: Box<crate::models::GetXrpRippleBlockDetailsByBlockHeightRiTotalFees>,
    /// Represents the total number of all transactions as part of this block.
    #[serde(rename = "transactionsCount")]
    pub transactions_count: i32,
}

impl GetXrpRippleBlockDetailsByBlockHashRi {
    pub fn new(block_hash: String, block_height: i32, next_block_hash: String, previous_block_hash: String, timestamp: i32, total_coins: crate::models::GetXrpRippleBlockDetailsByBlockHashRiTotalCoins, total_fees: crate::models::GetXrpRippleBlockDetailsByBlockHeightRiTotalFees, transactions_count: i32) -> GetXrpRippleBlockDetailsByBlockHashRi {
        GetXrpRippleBlockDetailsByBlockHashRi {
            block_hash,
            block_height,
            next_block_hash,
            previous_block_hash,
            timestamp,
            total_coins: Box::new(total_coins),
            total_fees: Box::new(total_fees),
            transactions_count,
        }
    }
}


