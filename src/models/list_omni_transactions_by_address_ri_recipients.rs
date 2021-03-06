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
pub struct ListOmniTransactionsByAddressRiRecipients {
    /// Represents the hash of the address that receives the funds.
    #[serde(rename = "address")]
    pub address: String,
    /// Defines the amount of the received funds as a string.
    #[serde(rename = "amount")]
    pub amount: String,
}

impl ListOmniTransactionsByAddressRiRecipients {
    pub fn new(address: String, amount: String) -> ListOmniTransactionsByAddressRiRecipients {
        ListOmniTransactionsByAddressRiRecipients {
            address,
            amount,
        }
    }
}


