/*
 * CryptoAPIs
 *
 * Crypto APIs is a complex and innovative infrastructure layer that radically simplifies the development of any Blockchain and Crypto related applications. Organized around REST, Crypto APIs can assist both novice Bitcoin/Ethereum enthusiasts and crypto experts with the development of their blockchain applications. Crypto APIs provides unified endpoints and data, raw data, automatic tokens and coins forwardings, callback functionalities, and much more.
 *
 * The version of the OpenAPI document: 2021-03-20
 * Contact: developers@cryptoapis.io
 * Generated by: https://openapi-generator.tech
 */

/// ListLatestMinedBlocksRibsb : Bitcoin



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListLatestMinedBlocksRibsb {
    /// A sub-unit of BCH equal to 0.000001 BCH, or 100 Satoshi, and is the same as microbitcoincash (μBCH). Bits have two-decimal precision.
    #[serde(rename = "bits")]
    pub bits: String,
    /// Represents a hexadecimal number of all the hashes necessary to produce the current chain. E.g., when converting 0000000000000000000000000000000000000000000086859f7a841475b236fd to a decimal you get 635262017308958427068157 hashes, or 635262 exahashes.
    #[serde(rename = "chainwork")]
    pub chainwork: String,
    /// Represents a mathematical value of how hard it is to find a valid hash for this block.
    #[serde(rename = "difficulty", skip_serializing_if = "Option::is_none")]
    pub difficulty: Option<String>,
    /// Defines the single and final (root) node of a Merkle tree. It is the combined hash of all transactions' hashes that are part of a blockchain block.
    #[serde(rename = "merkleRoot")]
    pub merkle_root: String,
    /// Represents a random value that can be adjusted to satisfy the proof of work
    #[serde(rename = "nonce", skip_serializing_if = "Option::is_none")]
    pub nonce: Option<i64>,
    /// Represents the block size
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    /// Defines the numeric representation of the block size excluding the witness data.
    #[serde(rename = "strippedSize")]
    pub stripped_size: i32,
    /// Represents the version of the specific block on the blockchain.
    #[serde(rename = "version")]
    pub version: i32,
    /// Is the hexadecimal string representation of the block's version.
    #[serde(rename = "versionHex")]
    pub version_hex: String,
    /// Represents a measurement to compare the size of different transactions to each other in proportion to the block size limit.
    #[serde(rename = "weight")]
    pub weight: i32,
}

impl ListLatestMinedBlocksRibsb {
    /// Bitcoin
    pub fn new(bits: String, chainwork: String, merkle_root: String, stripped_size: i32, version: i32, version_hex: String, weight: i32) -> ListLatestMinedBlocksRibsb {
        ListLatestMinedBlocksRibsb {
            bits,
            chainwork,
            difficulty: None,
            merkle_root,
            nonce: None,
            size: None,
            stripped_size,
            version,
            version_hex,
            weight,
        }
    }
}


