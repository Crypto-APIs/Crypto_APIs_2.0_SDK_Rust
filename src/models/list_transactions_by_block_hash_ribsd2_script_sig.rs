/*
 * CryptoAPIs
 *
 * Crypto APIs is a complex and innovative infrastructure layer that radically simplifies the development of any Blockchain and Crypto related applications. Organized around REST, Crypto APIs can assist both novice Bitcoin/Ethereum enthusiasts and crypto experts with the development of their blockchain applications. Crypto APIs provides unified endpoints and data, raw data, automatic tokens and coins forwardings, callback functionalities, and much more.
 *
 * The version of the OpenAPI document: 2021-03-20
 * Contact: developers@cryptoapis.io
 * Generated by: https://openapi-generator.tech
 */

/// ListTransactionsByBlockHashRibsd2ScriptSig : Specifies the required signatures.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListTransactionsByBlockHashRibsd2ScriptSig {
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

impl ListTransactionsByBlockHashRibsd2ScriptSig {
    /// Specifies the required signatures.
    pub fn new(asm: String, hex: String, _type: String) -> ListTransactionsByBlockHashRibsd2ScriptSig {
        ListTransactionsByBlockHashRibsd2ScriptSig {
            asm,
            hex,
            _type,
        }
    }
}


