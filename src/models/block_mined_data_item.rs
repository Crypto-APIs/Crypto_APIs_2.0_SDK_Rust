/*
 * CryptoAPIs
 *
 * Crypto APIs 2.0 is a complex and innovative infrastructure layer that radically simplifies the development of any Blockchain and Crypto related applications. Organized around REST, Crypto APIs 2.0 can assist both novice Bitcoin/Ethereum enthusiasts and crypto experts with the development of their blockchain applications. Crypto APIs 2.0 provides unified endpoints and data, raw data, automatic tokens and coins forwardings, callback functionalities, and much more.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: developers@cryptoapis.io
 * Generated by: https://openapi-generator.tech
 */

/// BlockMinedDataItem : Defines an `item` as one result.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BlockMinedDataItem {
    /// Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc.
    #[serde(rename = "blockchain")]
    pub blockchain: String,
    /// Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\", \"rinkeby\" are test networks.
    #[serde(rename = "network")]
    pub network: String,
    /// Defines the number of blocks in the blockchain preceding this specific block.
    #[serde(rename = "height")]
    pub height: i32,
    /// Represents the hash of the block's header, i.e. an output that has a fixed length.
    #[serde(rename = "hash")]
    pub hash: String,
    /// Defines the exact date/time when this block was mined in seconds since Unix Epoch time.
    #[serde(rename = "timestamp")]
    pub timestamp: i32,
}

impl BlockMinedDataItem {
    /// Defines an `item` as one result.
    pub fn new(blockchain: String, network: String, height: i32, hash: String, timestamp: i32) -> BlockMinedDataItem {
        BlockMinedDataItem {
            blockchain,
            network,
            height,
            hash,
            timestamp,
        }
    }
}


