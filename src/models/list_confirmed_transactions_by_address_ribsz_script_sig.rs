/*
 * CryptoAPIs
 *
 * Crypto APIs 2.0 is a complex and innovative infrastructure layer that radically simplifies the development of any Blockchain and Crypto related applications. Organized around REST, Crypto APIs 2.0 can assist both novice Bitcoin/Ethereum enthusiasts and crypto experts with the development of their blockchain applications. Crypto APIs 2.0 provides unified endpoints and data, raw data, automatic tokens and coins forwardings, callback functionalities, and much more.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: developers@cryptoapis.io
 * Generated by: https://openapi-generator.tech
 */

/// ListConfirmedTransactionsByAddressRibszScriptSig : Specifies the required signatures.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListConfirmedTransactionsByAddressRibszScriptSig {
    /// The asm strands for assembly, which is the symbolic representation of the Bitcoin's Script language op-codes.
    #[serde(rename = "asm")]
    pub asm: String,
    /// Represents the hex of the public key of the address.
    #[serde(rename = "hex")]
    pub hex: String,
    /// Represents the script type of the reference transaction identifier.
    #[serde(rename = "type")]
    pub _type: String,
}

impl ListConfirmedTransactionsByAddressRibszScriptSig {
    /// Specifies the required signatures.
    pub fn new(asm: String, hex: String, _type: String) -> ListConfirmedTransactionsByAddressRibszScriptSig {
        ListConfirmedTransactionsByAddressRibszScriptSig {
            asm,
            hex,
            _type,
        }
    }
}


