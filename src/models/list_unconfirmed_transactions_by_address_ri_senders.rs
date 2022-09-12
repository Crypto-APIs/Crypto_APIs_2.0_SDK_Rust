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
pub struct ListUnconfirmedTransactionsByAddressRiSenders {
    /// Represents the address of the sender.
    #[serde(rename = "address")]
    pub address: String,
    /// Represents the total amount sent by this address including the fee.
    #[serde(rename = "amount")]
    pub amount: String,
}

impl ListUnconfirmedTransactionsByAddressRiSenders {
    pub fn new(address: String, amount: String) -> ListUnconfirmedTransactionsByAddressRiSenders {
        ListUnconfirmedTransactionsByAddressRiSenders {
            address,
            amount,
        }
    }
}


