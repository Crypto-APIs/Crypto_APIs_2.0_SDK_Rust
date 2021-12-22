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
pub struct GetWalletAssetDetailsRiNonFungibleTokens {
    /// Defines the specific token identifier. For Bitcoin-based transactions it should be the propertyId and for Ethereum-based transactions - the contract.
    #[serde(rename = "identifier")]
    pub identifier: String,
    /// Defines the symbol of the non-fungible tokens.
    #[serde(rename = "symbol")]
    pub symbol: String,
    /// Represents tokens' unique identifier.
    #[serde(rename = "tokenId")]
    pub token_id: String,
    /// Defines the specific token type.
    #[serde(rename = "type")]
    pub _type: String,
}

impl GetWalletAssetDetailsRiNonFungibleTokens {
    pub fn new(identifier: String, symbol: String, token_id: String, _type: String) -> GetWalletAssetDetailsRiNonFungibleTokens {
        GetWalletAssetDetailsRiNonFungibleTokens {
            identifier,
            symbol,
            token_id,
            _type,
        }
    }
}


