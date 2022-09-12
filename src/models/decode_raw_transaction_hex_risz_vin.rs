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
pub struct DecodeRawTransactionHexRiszVin {
    /// Represents the addresses which send/receive the amount.
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Represents the transaction inputs' indentifier.
    #[serde(rename = "inputHash", skip_serializing_if = "Option::is_none")]
    pub input_hash: Option<String>,
    /// Defines the output index of a transaction.
    #[serde(rename = "outputIndex", skip_serializing_if = "Option::is_none")]
    pub output_index: Option<String>,
    #[serde(rename = "scriptSig")]
    pub script_sig: Box<crate::models::DecodeRawTransactionHexRiszScriptSig>,
    /// Represents the script sequence number.
    #[serde(rename = "sequence", skip_serializing_if = "Option::is_none")]
    pub sequence: Option<String>,
}

impl DecodeRawTransactionHexRiszVin {
    pub fn new(script_sig: crate::models::DecodeRawTransactionHexRiszScriptSig) -> DecodeRawTransactionHexRiszVin {
        DecodeRawTransactionHexRiszVin {
            address: None,
            input_hash: None,
            output_index: None,
            script_sig: Box::new(script_sig),
            sequence: None,
        }
    }
}


