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
pub struct ListWalletTransactionsRiFee {
    /// Defines the fee for the transaction.
    #[serde(rename = "amount")]
    pub amount: String,
    /// Defines the converted amount of the transaction's fee.
    #[serde(rename = "convertedAmount")]
    pub converted_amount: String,
    /// Defines the exchange rate for the transaction's unit.
    #[serde(rename = "exchangeRateUnit")]
    pub exchange_rate_unit: String,
    /// Defines the unit of the transaction's fee.
    #[serde(rename = "symbol")]
    pub symbol: String,
}

impl ListWalletTransactionsRiFee {
    pub fn new(amount: String, converted_amount: String, exchange_rate_unit: String, symbol: String) -> ListWalletTransactionsRiFee {
        ListWalletTransactionsRiFee {
            amount,
            converted_amount,
            exchange_rate_unit,
            symbol,
        }
    }
}


