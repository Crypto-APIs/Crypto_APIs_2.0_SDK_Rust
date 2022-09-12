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
pub struct ListConfirmedTransactionsByAddressRibszVin {
    #[serde(rename = "addresses")]
    pub addresses: Vec<String>,
    /// Represents the coinbase hex.
    #[serde(rename = "coinbase")]
    pub coinbase: String,
    #[serde(rename = "scriptSig")]
    pub script_sig: Box<crate::models::GetTransactionDetailsByTransactionIdribszScriptSig>,
    /// Represents the script sequence number.
    #[serde(rename = "sequence")]
    pub sequence: i64,
    /// Represents the reference transaction identifier.
    #[serde(rename = "txid")]
    pub txid: String,
    #[serde(rename = "txinwitness")]
    pub txinwitness: Vec<String>,
    /// Defines the specific amount.
    #[serde(rename = "value")]
    pub value: String,
    /// It refers to the index of the output address of this transaction. The index starts from 0.
    #[serde(rename = "vout")]
    pub vout: i32,
}

impl ListConfirmedTransactionsByAddressRibszVin {
    pub fn new(addresses: Vec<String>, coinbase: String, script_sig: crate::models::GetTransactionDetailsByTransactionIdribszScriptSig, sequence: i64, txid: String, txinwitness: Vec<String>, value: String, vout: i32) -> ListConfirmedTransactionsByAddressRibszVin {
        ListConfirmedTransactionsByAddressRibszVin {
            addresses,
            coinbase,
            script_sig: Box::new(script_sig),
            sequence,
            txid,
            txinwitness,
            value,
            vout,
        }
    }
}


