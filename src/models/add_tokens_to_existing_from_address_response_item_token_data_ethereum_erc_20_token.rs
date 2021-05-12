/*
 * CryptoAPIs
 *
 * Crypto APIs 2.0 is a complex and innovative infrastructure layer that radically simplifies the development of any Blockchain and Crypto related applications. Organized around REST, Crypto APIs 2.0 can assist both novice Bitcoin/Ethereum enthusiasts and crypto experts with the development of their blockchain applications. Crypto APIs 2.0 provides unified endpoints and data, raw data, automatic tokens and coins forwardings, callback functionalities, and much more.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: developers@cryptoapis.io
 * Generated by: https://openapi-generator.tech
 */

/// AddTokensToExistingFromAddressResponseItemTokenDataEthereumErc20Token : Ethereum erc-20 token



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddTokensToExistingFromAddressResponseItemTokenDataEthereumErc20Token {
    /// Token contract address to be transferred
    #[serde(rename = "contractAddress")]
    pub contract_address: String,
}

impl AddTokensToExistingFromAddressResponseItemTokenDataEthereumErc20Token {
    /// Ethereum erc-20 token
    pub fn new(contract_address: String) -> AddTokensToExistingFromAddressResponseItemTokenDataEthereumErc20Token {
        AddTokensToExistingFromAddressResponseItemTokenDataEthereumErc20Token {
            contract_address,
        }
    }
}

