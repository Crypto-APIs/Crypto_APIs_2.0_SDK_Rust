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
pub struct GetWalletTransactionDetailsByTransactionIdribsbcVout {
    /// Defines whether the output is spent or not.
    #[serde(rename = "isSpent")]
    pub is_spent: bool,
    #[serde(rename = "scriptPubKey")]
    pub script_pub_key: Box<crate::models::GetWalletTransactionDetailsByTransactionIdribsbcScriptPubKey>,
    /// Represents the sent/received amount.
    #[serde(rename = "value")]
    pub value: String,
}

impl GetWalletTransactionDetailsByTransactionIdribsbcVout {
    pub fn new(is_spent: bool, script_pub_key: crate::models::GetWalletTransactionDetailsByTransactionIdribsbcScriptPubKey, value: String) -> GetWalletTransactionDetailsByTransactionIdribsbcVout {
        GetWalletTransactionDetailsByTransactionIdribsbcVout {
            is_spent,
            script_pub_key: Box::new(script_pub_key),
            value,
        }
    }
}


