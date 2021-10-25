/*
 * CryptoAPIs
 *
 * Crypto APIs 2.0 is a complex and innovative infrastructure layer that radically simplifies the development of any Blockchain and Crypto related applications. Organized around REST, Crypto APIs 2.0 can assist both novice Bitcoin/Ethereum enthusiasts and crypto experts with the development of their blockchain applications. Crypto APIs 2.0 provides unified endpoints and data, raw data, automatic tokens and coins forwardings, callback functionalities, and much more.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: developers@cryptoapis.io
 * Generated by: https://openapi-generator.tech
 */

/// ListUnconfirmedTransactionsByAddressRibsec : Ethereum Classic



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListUnconfirmedTransactionsByAddressRibsec {
    #[serde(rename = "fee")]
    pub fee: Box<crate::models::ListUnconfirmedTransactionsByAddressRibsecFee>,
    /// Represents the amount of gas used by this specific transaction alone.
    #[serde(rename = "gasLimit")]
    pub gas_limit: String,
    #[serde(rename = "gasPrice")]
    pub gas_price: Box<crate::models::ListUnconfirmedTransactionsByAddressRibsecGasPrice>,
    /// Represents the sequential running number for an address, starting from 0 for the first transaction. E.g., if the nonce of a transaction is 10, it would be the 11th transaction sent from the sender's address.
    #[serde(rename = "nonce")]
    pub nonce: i32,
    /// String representation of the transaction status
    #[serde(rename = "transactionStatus")]
    pub transaction_status: String,
}

impl ListUnconfirmedTransactionsByAddressRibsec {
    /// Ethereum Classic
    pub fn new(fee: crate::models::ListUnconfirmedTransactionsByAddressRibsecFee, gas_limit: String, gas_price: crate::models::ListUnconfirmedTransactionsByAddressRibsecGasPrice, nonce: i32, transaction_status: String) -> ListUnconfirmedTransactionsByAddressRibsec {
        ListUnconfirmedTransactionsByAddressRibsec {
            fee: Box::new(fee),
            gas_limit,
            gas_price: Box::new(gas_price),
            nonce,
            transaction_status,
        }
    }
}


