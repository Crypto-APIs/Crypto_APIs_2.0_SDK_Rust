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
pub struct DeriveHdWalletXPubYPubZPubChangeOrReceivingAddressesRiAddresses {
    /// Represents the public address, which is a compressed and shortened form of a public key.
    #[serde(rename = "address")]
    pub address: String,
    /// Represents the output index. It refers to the UTXO sequence in the transaction outputs (vout).
    #[serde(rename = "index")]
    pub index: i64,
}

impl DeriveHdWalletXPubYPubZPubChangeOrReceivingAddressesRiAddresses {
    pub fn new(address: String, index: i64) -> DeriveHdWalletXPubYPubZPubChangeOrReceivingAddressesRiAddresses {
        DeriveHdWalletXPubYPubZPubChangeOrReceivingAddressesRiAddresses {
            address,
            index,
        }
    }
}


