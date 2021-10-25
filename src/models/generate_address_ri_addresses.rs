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
pub struct GenerateAddressRiAddresses {
    /// Represents the specific address that will be generated.
    #[serde(rename = "address")]
    pub address: String,
    /// Defines the form of transaction that is used. Keep in mind that we support more than one type of formats for example: p2pkh p2sh p2wpkh etc. and they are generated simultaneously in the response of the Generate Address endpoint, depending on the blockchain and network that has been chosen. For more information about supported formats - please check \"What we support\" section
    #[serde(rename = "format")]
    pub format: String,
}

impl GenerateAddressRiAddresses {
    pub fn new(address: String, format: String) -> GenerateAddressRiAddresses {
        GenerateAddressRiAddresses {
            address,
            format,
        }
    }
}


