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
pub struct GetTransactionDetailsByTransactionIdribsd2Vout {
    /// Defines whether the output is spent or not.
    #[serde(rename = "isSpent")]
    pub is_spent: bool,
    #[serde(rename = "scriptPubKey")]
    pub script_pub_key: Box<crate::models::GetTransactionDetailsByTransactionIdribsd2ScriptPubKey>,
    /// Represents the sent/received amount.
    #[serde(rename = "value")]
    pub value: String,
}

impl GetTransactionDetailsByTransactionIdribsd2Vout {
    pub fn new(is_spent: bool, script_pub_key: crate::models::GetTransactionDetailsByTransactionIdribsd2ScriptPubKey, value: String) -> GetTransactionDetailsByTransactionIdribsd2Vout {
        GetTransactionDetailsByTransactionIdribsd2Vout {
            is_spent,
            script_pub_key: Box::new(script_pub_key),
            value,
        }
    }
}


