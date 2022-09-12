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
pub struct GetTransactionDetailsByTransactionIdFromCallbackRibszVout {
    /// Defines whether the transaction output has been spent or not.
    #[serde(rename = "isSpent")]
    pub is_spent: bool,
    #[serde(rename = "scriptPubKey")]
    pub script_pub_key: Box<crate::models::GetTransactionDetailsByTransactionIdFromCallbackRibszScriptPubKey>,
    /// Represents the specific amount.
    #[serde(rename = "value")]
    pub value: String,
}

impl GetTransactionDetailsByTransactionIdFromCallbackRibszVout {
    pub fn new(is_spent: bool, script_pub_key: crate::models::GetTransactionDetailsByTransactionIdFromCallbackRibszScriptPubKey, value: String) -> GetTransactionDetailsByTransactionIdFromCallbackRibszVout {
        GetTransactionDetailsByTransactionIdFromCallbackRibszVout {
            is_spent,
            script_pub_key: Box::new(script_pub_key),
            value,
        }
    }
}


