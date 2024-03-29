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
pub struct GetHdWalletXPubYPubZPubDetailsRi {
    /// Specifies the confirmed coins balance of the Wallet.
    #[serde(rename = "confirmedBalance")]
    pub confirmed_balance: String,
    /// Defines the total currency received to the Wallet.
    #[serde(rename = "totalReceived", skip_serializing_if = "Option::is_none")]
    pub total_received: Option<String>,
    /// Defines the total currency spent from the Wallet.
    #[serde(rename = "totalSpent", skip_serializing_if = "Option::is_none")]
    pub total_spent: Option<String>,
}

impl GetHdWalletXPubYPubZPubDetailsRi {
    pub fn new(confirmed_balance: String) -> GetHdWalletXPubYPubZPubDetailsRi {
        GetHdWalletXPubYPubZPubDetailsRi {
            confirmed_balance,
            total_received: None,
            total_spent: None,
        }
    }
}


