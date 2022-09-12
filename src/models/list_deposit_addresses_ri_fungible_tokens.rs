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
pub struct ListDepositAddressesRiFungibleTokens {
    /// Defines the amount of the fungible tokens.
    #[serde(rename = "amount")]
    pub amount: String,
    /// Defines the specific token identifier. For Bitcoin-based transactions it should be the propertyId and for Ethereum-based transactions - the contract.
    #[serde(rename = "identifier")]
    pub identifier: String,
    /// Defines the token's name as a string.
    #[serde(rename = "name")]
    pub name: String,
    /// Defines the symbol of the fungible tokens.
    #[serde(rename = "symbol")]
    pub symbol: String,
    /// Defines the decimals of the token, i.e. the number of digits that come after the decimal coma of the token.
    #[serde(rename = "tokenDecimals")]
    pub token_decimals: i64,
    /// Defines the specific token type.
    #[serde(rename = "type")]
    pub _type: String,
}

impl ListDepositAddressesRiFungibleTokens {
    pub fn new(amount: String, identifier: String, name: String, symbol: String, token_decimals: i64, _type: String) -> ListDepositAddressesRiFungibleTokens {
        ListDepositAddressesRiFungibleTokens {
            amount,
            identifier,
            name,
            symbol,
            token_decimals,
            _type,
        }
    }
}


