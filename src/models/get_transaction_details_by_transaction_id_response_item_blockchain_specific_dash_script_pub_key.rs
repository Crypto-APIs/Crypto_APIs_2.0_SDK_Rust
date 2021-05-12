/*
 * CryptoAPIs
 *
 * Crypto APIs 2.0 is a complex and innovative infrastructure layer that radically simplifies the development of any Blockchain and Crypto related applications. Organized around REST, Crypto APIs 2.0 can assist both novice Bitcoin/Ethereum enthusiasts and crypto experts with the development of their blockchain applications. Crypto APIs 2.0 provides unified endpoints and data, raw data, automatic tokens and coins forwardings, callback functionalities, and much more.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: developers@cryptoapis.io
 * Generated by: https://openapi-generator.tech
 */

/// GetTransactionDetailsByTransactionIdResponseItemBlockchainSpecificDashScriptPubKey : Represents the script public key.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetTransactionDetailsByTransactionIdResponseItemBlockchainSpecificDashScriptPubKey {
    #[serde(rename = "addresses")]
    pub addresses: Vec<String>,
    /// Represents the assembly of the script public key of the address.
    #[serde(rename = "asm")]
    pub asm: String,
    /// Represents the hex of the script public key of the address.
    #[serde(rename = "hex")]
    pub hex: String,
    /// Represents the required signatures.
    #[serde(rename = "reqSigs")]
    pub req_sigs: i32,
    /// Represents the script type.
    #[serde(rename = "type")]
    pub _type: String,
}

impl GetTransactionDetailsByTransactionIdResponseItemBlockchainSpecificDashScriptPubKey {
    /// Represents the script public key.
    pub fn new(addresses: Vec<String>, asm: String, hex: String, req_sigs: i32, _type: String) -> GetTransactionDetailsByTransactionIdResponseItemBlockchainSpecificDashScriptPubKey {
        GetTransactionDetailsByTransactionIdResponseItemBlockchainSpecificDashScriptPubKey {
            addresses,
            asm,
            hex,
            req_sigs,
            _type,
        }
    }
}


