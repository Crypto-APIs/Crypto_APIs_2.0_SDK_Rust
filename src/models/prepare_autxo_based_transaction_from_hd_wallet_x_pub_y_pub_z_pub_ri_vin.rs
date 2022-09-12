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
pub struct PrepareAutxoBasedTransactionFromHdWalletXPubYPubZPubRiVin {
    /// Representation of the address
    #[serde(rename = "address")]
    pub address: String,
    /// Representation of the change value
    #[serde(rename = "change", skip_serializing_if = "Option::is_none")]
    pub change: Option<i64>,
    /// Representation of the derivation index of the xpub address.
    #[serde(rename = "derivationIndex", skip_serializing_if = "Option::is_none")]
    pub derivation_index: Option<i64>,
    /// Representation of the output index
    #[serde(rename = "outputIndex")]
    pub output_index: i32,
    /// Representation of the satoshis value
    #[serde(rename = "satoshis")]
    pub satoshis: i64,
    /// Representation of the script string
    #[serde(rename = "script")]
    pub script: String,
    /// Representation of the hash that should be signed.
    #[serde(rename = "sighash")]
    pub sighash: String,
    /// Represents the reference transaction identifier.
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
}

impl PrepareAutxoBasedTransactionFromHdWalletXPubYPubZPubRiVin {
    pub fn new(address: String, output_index: i32, satoshis: i64, script: String, sighash: String, transaction_id: String) -> PrepareAutxoBasedTransactionFromHdWalletXPubYPubZPubRiVin {
        PrepareAutxoBasedTransactionFromHdWalletXPubYPubZPubRiVin {
            address,
            change: None,
            derivation_index: None,
            output_index,
            satoshis,
            script,
            sighash,
            transaction_id,
        }
    }
}

