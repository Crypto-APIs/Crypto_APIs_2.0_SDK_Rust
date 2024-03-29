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
pub struct GetFeeAddressDetailsRi {
    /// Represents the specific fee address, which is always automatically generated. Users must fund it.
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "balance")]
    pub balance: Box<crate::models::GetFeeAddressDetailsRiBalance>,
    /// Represents the minimum transfer amount of the currency in the `fromAddress` that can be allowed for an automatic forwarding.
    #[serde(rename = "minimumTransferAmount")]
    pub minimum_transfer_amount: String,
}

impl GetFeeAddressDetailsRi {
    pub fn new(address: String, balance: crate::models::GetFeeAddressDetailsRiBalance, minimum_transfer_amount: String) -> GetFeeAddressDetailsRi {
        GetFeeAddressDetailsRi {
            address,
            balance: Box::new(balance),
            minimum_transfer_amount,
        }
    }
}


