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
pub struct ListDepositAddressesRi {
    /// Specifies the specific address's unique string value.
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "confirmedBalance")]
    pub confirmed_balance: Box<crate::models::ListDepositAddressesRiConfirmedBalance>,
    /// Defines the specific UNIX time when the deposit address was created.
    #[serde(rename = "createdTimestamp")]
    pub created_timestamp: i32,
    /// Represents fungible tokens'es detailed information
    #[serde(rename = "fungibleTokens")]
    pub fungible_tokens: Vec<crate::models::ListDepositAddressesRiFungibleTokens>,
    /// Represents the index of the address in the wallet.
    #[serde(rename = "index")]
    pub index: String,
    /// Represents a custom tag that customers can set up for their Wallets and addresses. E.g. custom label named \"Special addresses\".
    #[serde(rename = "label")]
    pub label: String,
    /// Represents non-fungible tokens'es detailed information.
    #[serde(rename = "nonFungibleTokens")]
    pub non_fungible_tokens: Vec<crate::models::ListDepositAddressesRiNonFungibleTokens>,
}

impl ListDepositAddressesRi {
    pub fn new(address: String, confirmed_balance: crate::models::ListDepositAddressesRiConfirmedBalance, created_timestamp: i32, fungible_tokens: Vec<crate::models::ListDepositAddressesRiFungibleTokens>, index: String, label: String, non_fungible_tokens: Vec<crate::models::ListDepositAddressesRiNonFungibleTokens>) -> ListDepositAddressesRi {
        ListDepositAddressesRi {
            address,
            confirmed_balance: Box::new(confirmed_balance),
            created_timestamp,
            fungible_tokens,
            index,
            label,
            non_fungible_tokens,
        }
    }
}


