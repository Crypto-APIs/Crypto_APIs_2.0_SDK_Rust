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
pub struct GetBlockDetailsByBlockHeightRibs {
    /// Represents a mathematical value of how hard it is to find a valid hash for this block.
    #[serde(rename = "difficulty")]
    pub difficulty: String,
    /// Represents a random value that can be adjusted to satisfy the Proof of Work.
    #[serde(rename = "nonce")]
    pub nonce: String,
    /// Represents the total size of the block in Bytes.
    #[serde(rename = "size")]
    pub size: i32,
    /// Represents a specific sub-unit of Zcash. Bits have two-decimal precision
    #[serde(rename = "bits")]
    pub bits: String,
    /// Represents a hexadecimal number of all the hashes necessary to produce the current chain. E.g., when converting 0000000000000000000000000000000000000000000086859f7a841475b236fd to a decimal you get 635262017308958427068157 hashes, or 635262 exahashes.
    #[serde(rename = "chainwork")]
    pub chainwork: String,
    /// Defines the single and final (root) node of a Merkle tree. It is the combined hash of all transactions' hashes that are part of a blockchain block.
    #[serde(rename = "merkleRoot")]
    pub merkle_root: String,
    /// Defines the numeric representation of the block size excluding the witness data.
    #[serde(rename = "strippedSize")]
    pub stripped_size: i32,
    /// Represents the block version number.
    #[serde(rename = "version")]
    pub version: i32,
    /// Is the hexadecimal string representation of the block's version.
    #[serde(rename = "versionHex")]
    pub version_hex: String,
    /// Represents a measurement to compare the size of different transactions to each other in proportion to the block size limi
    #[serde(rename = "weight")]
    pub weight: i32,
    /// Represents any data that can be included by the miner in the block.
    #[serde(rename = "extraData")]
    pub extra_data: String,
    /// Defines the total gas limit of all transactions in the block.
    #[serde(rename = "gasLimit")]
    pub gas_limit: String,
    /// Represents the total amount of gas used by all transactions in this block.
    #[serde(rename = "gasUsed")]
    pub gas_used: String,
    /// Specifies the amount of time required for the block to be mined in seconds.
    #[serde(rename = "minedInSeconds")]
    pub mined_in_seconds: i32,
    /// Defines the combined hash of all uncles for a given parent.
    #[serde(rename = "sha3Uncles")]
    pub sha3_uncles: String,
    /// Defines the total difficulty of the chain until this block, i.e. how difficult it is for a specific miner to mine a new block.
    #[serde(rename = "totalDifficulty")]
    pub total_difficulty: String,
}

impl GetBlockDetailsByBlockHeightRibs {
    pub fn new(difficulty: String, nonce: String, size: i32, bits: String, chainwork: String, merkle_root: String, stripped_size: i32, version: i32, version_hex: String, weight: i32, extra_data: String, gas_limit: String, gas_used: String, mined_in_seconds: i32, sha3_uncles: String, total_difficulty: String) -> GetBlockDetailsByBlockHeightRibs {
        GetBlockDetailsByBlockHeightRibs {
            difficulty,
            nonce,
            size,
            bits,
            chainwork,
            merkle_root,
            stripped_size,
            version,
            version_hex,
            weight,
            extra_data,
            gas_limit,
            gas_used,
            mined_in_seconds,
            sha3_uncles,
            total_difficulty,
        }
    }
}


