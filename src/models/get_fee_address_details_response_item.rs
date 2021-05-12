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
pub struct GetFeeAddressDetailsResponseItem {
    /// Represents the specific fee address, which is always automatically generated. Users must fund it.
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "balance")]
    pub balance: Box<crate::models::GetFeeAddressDetailsResponseItemBalance>,
    /// Represents the minimum transfer amount of the currency in the `fromAddress` that can be allowed for an automatic forwarding.
    #[serde(rename = "minimumTransferAmount")]
    pub minimum_transfer_amount: String,
}

impl GetFeeAddressDetailsResponseItem {
    pub fn new(address: String, balance: crate::models::GetFeeAddressDetailsResponseItemBalance, minimum_transfer_amount: String) -> GetFeeAddressDetailsResponseItem {
        GetFeeAddressDetailsResponseItem {
            address,
            balance: Box::new(balance),
            minimum_transfer_amount,
        }
    }
}


