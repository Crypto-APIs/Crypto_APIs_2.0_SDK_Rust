/*
 * CryptoAPIs
 *
 * Crypto APIs 2.0 is a complex and innovative infrastructure layer that radically simplifies the development of any Blockchain and Crypto related applications. Organized around REST, Crypto APIs 2.0 can assist both novice Bitcoin/Ethereum enthusiasts and crypto experts with the development of their blockchain applications. Crypto APIs 2.0 provides unified endpoints and data, raw data, automatic tokens and coins forwardings, callback functionalities, and much more.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: developers@cryptoapis.io
 * Generated by: https://openapi-generator.tech
 */

/// AddressCoinsTransactionConfirmedEachConfirmationDataItemMinedInBlock : Refers to the specific block the transaction was mined in.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddressCoinsTransactionConfirmedEachConfirmationDataItemMinedInBlock {
    /// Defines the number of blocks in the blockchain preceding this specific block.
    #[serde(rename = "height")]
    pub height: i32,
    /// Represents the hash of the block's header, i.e. an output that has a fixed length.
    #[serde(rename = "hash")]
    pub hash: String,
    /// Defines the exact date/time when this transaction was mined in seconds since Unix Epoch time.
    #[serde(rename = "timestamp")]
    pub timestamp: i32,
}

impl AddressCoinsTransactionConfirmedEachConfirmationDataItemMinedInBlock {
    /// Refers to the specific block the transaction was mined in.
    pub fn new(height: i32, hash: String, timestamp: i32) -> AddressCoinsTransactionConfirmedEachConfirmationDataItemMinedInBlock {
        AddressCoinsTransactionConfirmedEachConfirmationDataItemMinedInBlock {
            height,
            hash,
            timestamp,
        }
    }
}

