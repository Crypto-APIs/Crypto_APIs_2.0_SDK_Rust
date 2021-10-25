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
pub struct GetLatestMinedZilliqaBlockRi {
    /// Represents the hash of the block, which is its unique identifier. It represents a cryptographic digital fingerprint made by hashing the block header twice through the SHA256 algorithm.
    #[serde(rename = "blockHash")]
    pub block_hash: String,
    /// Represents the number of blocks in the blockchain preceding this specific block. Block numbers have no gaps. A blockchain usually starts with block 0 called the \"Genesis block\".
    #[serde(rename = "blockHeight")]
    pub block_height: i32,
    /// Defines how difficult it is for a specific miner to mine the block.
    #[serde(rename = "difficulty")]
    pub difficulty: String,
    /// Represents the Directory Service block which contains metadata about the miners who participate in the consensus protocol.
    #[serde(rename = "dsBlock")]
    pub ds_block: i32,
    /// Defines how difficult it is to mine the dsBlocks.
    #[serde(rename = "dsDifficulty")]
    pub ds_difficulty: String,
    /// Represents a part of the DS Committee which leads the consensus protocol for the epoch.
    #[serde(rename = "dsLeader")]
    pub ds_leader: String,
    /// Represents the maximum amount of gas allowed in the block in order to determine how many transactions it can fit.
    #[serde(rename = "gasLimit")]
    pub gas_limit: i32,
    /// Defines how much of the gas for the block has been used.
    #[serde(rename = "gasUsed")]
    pub gas_used: i32,
    #[serde(rename = "microBlocks")]
    pub micro_blocks: Vec<String>,
    /// Represents the hash of the previous block, also known as the parent block.
    #[serde(rename = "previousBlockHash")]
    pub previous_block_hash: String,
    /// Defines the exact date/time when this block was mined in Unix Timestamp.
    #[serde(rename = "timestamp")]
    pub timestamp: i32,
    /// Represents the total number of all transactions as part of this block.
    #[serde(rename = "transactionsCount")]
    pub transactions_count: i32,
}

impl GetLatestMinedZilliqaBlockRi {
    pub fn new(block_hash: String, block_height: i32, difficulty: String, ds_block: i32, ds_difficulty: String, ds_leader: String, gas_limit: i32, gas_used: i32, micro_blocks: Vec<String>, previous_block_hash: String, timestamp: i32, transactions_count: i32) -> GetLatestMinedZilliqaBlockRi {
        GetLatestMinedZilliqaBlockRi {
            block_hash,
            block_height,
            difficulty,
            ds_block,
            ds_difficulty,
            ds_leader,
            gas_limit,
            gas_used,
            micro_blocks,
            previous_block_hash,
            timestamp,
            transactions_count,
        }
    }
}


