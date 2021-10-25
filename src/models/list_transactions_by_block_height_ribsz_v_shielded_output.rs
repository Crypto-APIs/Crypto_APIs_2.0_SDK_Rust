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
pub struct ListTransactionsByBlockHeightRibszVShieldedOutput {
    /// Represents the 𝑢-coordinate of the note commitment for the output note.
    #[serde(rename = "cmu")]
    pub cmu: String,
    /// Defines a value commitment to the value of the input note.
    #[serde(rename = "cv")]
    pub cv: String,
    /// Represents a ciphertext component for the encrypted output note.
    #[serde(rename = "encCipherText")]
    pub enc_cipher_text: String,
    /// Represents an encoding of an ephemeral Jubjub public key.
    #[serde(rename = "ephemeralKey")]
    pub ephemeral_key: String,
    /// Represents a ciphertext component that allows the holder of the outgoing cipher key to recover the diversified transmission key pkd and ephemeral private key esk, hence the entire note plaintext.
    #[serde(rename = "outCipherText")]
    pub out_cipher_text: String,
    /// Represents the proof
    #[serde(rename = "proof")]
    pub proof: String,
}

impl ListTransactionsByBlockHeightRibszVShieldedOutput {
    pub fn new(cmu: String, cv: String, enc_cipher_text: String, ephemeral_key: String, out_cipher_text: String, proof: String) -> ListTransactionsByBlockHeightRibszVShieldedOutput {
        ListTransactionsByBlockHeightRibszVShieldedOutput {
            cmu,
            cv,
            enc_cipher_text,
            ephemeral_key,
            out_cipher_text,
            proof,
        }
    }
}


