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
pub struct ListBlockchainEventsSubscriptionsRiDeactivationReasons {
    /// Defines the deactivation reason as a message.
    #[serde(rename = "reason")]
    pub reason: String,
    /// Represents the time of the subscription deactivation.
    #[serde(rename = "timestamp")]
    pub timestamp: i32,
}

impl ListBlockchainEventsSubscriptionsRiDeactivationReasons {
    pub fn new(reason: String, timestamp: i32) -> ListBlockchainEventsSubscriptionsRiDeactivationReasons {
        ListBlockchainEventsSubscriptionsRiDeactivationReasons {
            reason,
            timestamp,
        }
    }
}


