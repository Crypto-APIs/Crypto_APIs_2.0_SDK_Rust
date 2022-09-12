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
pub struct PrepareAutxoBasedTransactionFromHdWalletXPubYPubZPubRbDataItemFee {
    /// Represents the fee address
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Representation of the exact amount value
    #[serde(rename = "exactAmount", skip_serializing_if = "Option::is_none")]
    pub exact_amount: Option<String>,
    /// Represents the fee priority
    #[serde(rename = "priority", skip_serializing_if = "Option::is_none")]
    pub priority: Option<Priority>,
}

impl PrepareAutxoBasedTransactionFromHdWalletXPubYPubZPubRbDataItemFee {
    pub fn new() -> PrepareAutxoBasedTransactionFromHdWalletXPubYPubZPubRbDataItemFee {
        PrepareAutxoBasedTransactionFromHdWalletXPubYPubZPubRbDataItemFee {
            address: None,
            exact_amount: None,
            priority: None,
        }
    }
}

/// Represents the fee priority
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Priority {
    #[serde(rename = "slow")]
    Slow,
    #[serde(rename = "standard")]
    Standard,
    #[serde(rename = "fast")]
    Fast,
}
