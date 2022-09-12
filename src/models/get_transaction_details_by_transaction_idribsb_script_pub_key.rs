/*
 * CryptoAPIs
 *
 * Crypto APIs is a complex and innovative infrastructure layer that radically simplifies the development of any Blockchain and Crypto related applications. Organized around REST, Crypto APIs can assist both novice Bitcoin/Ethereum enthusiasts and crypto experts with the development of their blockchain applications. Crypto APIs provides unified endpoints and data, raw data, automatic tokens and coins forwardings, callback functionalities, and much more.
 *
 * The version of the OpenAPI document: 2021-03-20
 * Contact: developers@cryptoapis.io
 * Generated by: https://openapi-generator.tech
 */

/// GetTransactionDetailsByTransactionIdribsbScriptPubKey : Represents the script public key.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetTransactionDetailsByTransactionIdribsbScriptPubKey {
    #[serde(rename = "addresses")]
    pub addresses: Vec<String>,
    /// Represents the assembly of the script public key of the address.
    #[serde(rename = "asm")]
    pub asm: String,
    /// Represents the hex of the script public key of the address.
    #[serde(rename = "hex")]
    pub hex: String,
    /// Represents the required signatures.
    #[serde(rename = "reqSigs", skip_serializing_if = "Option::is_none")]
    pub req_sigs: Option<i32>,
    /// Represents the script type.
    #[serde(rename = "type")]
    pub _type: String,
}

impl GetTransactionDetailsByTransactionIdribsbScriptPubKey {
    /// Represents the script public key.
    pub fn new(addresses: Vec<String>, asm: String, hex: String, _type: String) -> GetTransactionDetailsByTransactionIdribsbScriptPubKey {
        GetTransactionDetailsByTransactionIdribsbScriptPubKey {
            addresses,
            asm,
            hex,
            req_sigs: None,
            _type,
        }
    }
}


