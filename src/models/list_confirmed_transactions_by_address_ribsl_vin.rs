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
pub struct ListConfirmedTransactionsByAddressRibslVin {
    #[serde(rename = "addresses")]
    pub addresses: Vec<String>,
    /// Represents the coinbase hex.
    #[serde(rename = "coinbase", skip_serializing_if = "Option::is_none")]
    pub coinbase: Option<String>,
    #[serde(rename = "scriptSig")]
    pub script_sig: Box<crate::models::ListConfirmedTransactionsByAddressRibslScriptSig>,
    /// Represents the script sequence number.
    #[serde(rename = "sequence")]
    pub sequence: String,
    /// Represents the reference transaction identifier.
    #[serde(rename = "txid")]
    pub txid: String,
    #[serde(rename = "txinwitness")]
    pub txinwitness: Vec<String>,
    /// Represents the sent/received amount.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Defines the vout of the transaction output, i.e. which output to spend.
    #[serde(rename = "vout", skip_serializing_if = "Option::is_none")]
    pub vout: Option<i32>,
}

impl ListConfirmedTransactionsByAddressRibslVin {
    pub fn new(addresses: Vec<String>, script_sig: crate::models::ListConfirmedTransactionsByAddressRibslScriptSig, sequence: String, txid: String, txinwitness: Vec<String>) -> ListConfirmedTransactionsByAddressRibslVin {
        ListConfirmedTransactionsByAddressRibslVin {
            addresses,
            coinbase: None,
            script_sig: Box::new(script_sig),
            sequence,
            txid,
            txinwitness,
            value: None,
            vout: None,
        }
    }
}


