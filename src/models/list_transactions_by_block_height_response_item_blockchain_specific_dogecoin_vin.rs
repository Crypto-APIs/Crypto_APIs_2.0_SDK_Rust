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
pub struct ListTransactionsByBlockHeightResponseItemBlockchainSpecificDogecoinVin {
    #[serde(rename = "addresses")]
    pub addresses: Vec<String>,
    /// Represents the coinbase hex.
    #[serde(rename = "coinbase", skip_serializing_if = "Option::is_none")]
    pub coinbase: Option<String>,
    #[serde(rename = "scriptSig")]
    pub script_sig: Box<crate::models::GetTransactionDetailsByTransactionIdResponseItemBlockchainSpecificDogecoinScriptSig>,
    /// Represents the script sequence number.
    #[serde(rename = "sequence")]
    pub sequence: String,
    /// Represents the reference transaction identifier.
    #[serde(rename = "txid", skip_serializing_if = "Option::is_none")]
    pub txid: Option<String>,
    #[serde(rename = "txinwitness")]
    pub txinwitness: Vec<String>,
    /// Represents the sent/received amount.
    #[serde(rename = "value")]
    pub value: String,
    /// It refers to the index of the output address of this transaction. The index starts from 0.
    #[serde(rename = "vout")]
    pub vout: i32,
}

impl ListTransactionsByBlockHeightResponseItemBlockchainSpecificDogecoinVin {
    pub fn new(addresses: Vec<String>, script_sig: crate::models::GetTransactionDetailsByTransactionIdResponseItemBlockchainSpecificDogecoinScriptSig, sequence: String, txinwitness: Vec<String>, value: String, vout: i32) -> ListTransactionsByBlockHeightResponseItemBlockchainSpecificDogecoinVin {
        ListTransactionsByBlockHeightResponseItemBlockchainSpecificDogecoinVin {
            addresses,
            coinbase: None,
            script_sig: Box::new(script_sig),
            sequence,
            txid: None,
            txinwitness,
            value,
            vout,
        }
    }
}


