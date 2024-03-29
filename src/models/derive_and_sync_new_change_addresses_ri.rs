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
pub struct DeriveAndSyncNewChangeAddressesRi {
    /// Represents the public address, which is a compressed and shortened form of a public key.
    #[serde(rename = "address")]
    pub address: String,
    /// Represents the format of the address.
    #[serde(rename = "addressFormat")]
    pub address_format: String,
    /// Defines the address type.
    #[serde(rename = "addressType")]
    pub address_type: AddressType,
    /// Represents the derivation type.
    #[serde(rename = "derivationType")]
    pub derivation_type: DerivationType,
    /// Represents the output index. It refers to the UTXO sequence in the transaction outputs (vout).
    #[serde(rename = "index")]
    pub index: String,
}

impl DeriveAndSyncNewChangeAddressesRi {
    pub fn new(address: String, address_format: String, address_type: AddressType, derivation_type: DerivationType, index: String) -> DeriveAndSyncNewChangeAddressesRi {
        DeriveAndSyncNewChangeAddressesRi {
            address,
            address_format,
            address_type,
            derivation_type,
            index,
        }
    }
}

/// Defines the address type.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AddressType {
    #[serde(rename = "change")]
    Change,
}
/// Represents the derivation type.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DerivationType {
    #[serde(rename = "account")]
    Account,
    #[serde(rename = "bip32")]
    Bip32,
}

