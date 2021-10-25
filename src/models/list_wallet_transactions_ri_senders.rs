/*
 * CryptoAPIs
 *
 * Crypto APIs 2.0 is a complex and innovative infrastructure layer that radically simplifies the development of any Blockchain and Crypto related applications. Organized around REST, Crypto APIs 2.0 can assist both novice Bitcoin/Ethereum enthusiasts and crypto experts with the development of their blockchain applications. Crypto APIs 2.0 provides unified endpoints and data, raw data, automatic tokens and coins forwardings, callback functionalities, and much more.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: developers@cryptoapis.io
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListWalletTransactionsRiSenders {
    /// Represents the address which sends this transaction. In UTXO-based protocols like Bitcoin there could be several senders while in account-based protocols like Ethereum there is always only one sender.
    #[serde(rename = "address")]
    pub address: String,
    /// Represents the total amount sent by this address including the fee.
    #[serde(rename = "amount")]
    pub amount: String,
    /// Defines a plain text string as a label for the sender.
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

impl ListWalletTransactionsRiSenders {
    pub fn new(address: String, amount: String) -> ListWalletTransactionsRiSenders {
        ListWalletTransactionsRiSenders {
            address,
            amount,
            label: None,
        }
    }
}


