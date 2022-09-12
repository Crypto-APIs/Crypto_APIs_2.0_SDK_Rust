/*
 * CryptoAPIs
 *
 * Crypto APIs is a complex and innovative infrastructure layer that radically simplifies the development of any Blockchain and Crypto related applications. Organized around REST, Crypto APIs can assist both novice Bitcoin/Ethereum enthusiasts and crypto experts with the development of their blockchain applications. Crypto APIs provides unified endpoints and data, raw data, automatic tokens and coins forwardings, callback functionalities, and much more.
 *
 * The version of the OpenAPI document: 2021-03-20
 * Contact: developers@cryptoapis.io
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetLatestMinedXrpRippleBlockRi {
    /// Represents the hash of the block, which is its unique identifier. It represents a cryptographic digital fingerprint made by hashing the block header twice through the SHA256 algorithm.
    #[serde(rename = "blockHash")]
    pub block_hash: String,
    /// Represents the number of blocks in the blockchain preceding this specific block. Block numbers have no gaps. A blockchain usually starts with block 0 called the \"Genesis block\".
    #[serde(rename = "blockHeight")]
    pub block_height: i32,
    /// Represents the hash of the previous block, also known as the parent block.
    #[serde(rename = "previousBlockHash")]
    pub previous_block_hash: String,
    /// Defines the exact date/time when this block was mined in Unix Timestamp.
    #[serde(rename = "timestamp")]
    pub timestamp: i32,
    /// Represents the total number of all transactions as part of this block.
    #[serde(rename = "transactionsCount")]
    pub transactions_count: i32,
    #[serde(rename = "totalCoins")]
    pub total_coins: Box<crate::models::GetLatestMinedXrpRippleBlockRiTotalCoins>,
    #[serde(rename = "totalFees")]
    pub total_fees: Box<crate::models::GetLatestMinedXrpRippleBlockRiTotalFees>,
}

impl GetLatestMinedXrpRippleBlockRi {
    pub fn new(block_hash: String, block_height: i32, previous_block_hash: String, timestamp: i32, transactions_count: i32, total_coins: crate::models::GetLatestMinedXrpRippleBlockRiTotalCoins, total_fees: crate::models::GetLatestMinedXrpRippleBlockRiTotalFees) -> GetLatestMinedXrpRippleBlockRi {
        GetLatestMinedXrpRippleBlockRi {
            block_hash,
            block_height,
            previous_block_hash,
            timestamp,
            transactions_count,
            total_coins: Box::new(total_coins),
            total_fees: Box::new(total_fees),
        }
    }
}


