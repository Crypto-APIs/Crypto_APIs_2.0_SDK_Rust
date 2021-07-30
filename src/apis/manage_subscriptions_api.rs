/*
 * CryptoAPIs
 *
 * Crypto APIs 2.0 is a complex and innovative infrastructure layer that radically simplifies the development of any Blockchain and Crypto related applications. Organized around REST, Crypto APIs 2.0 can assist both novice Bitcoin/Ethereum enthusiasts and crypto experts with the development of their blockchain applications. Crypto APIs 2.0 provides unified endpoints and data, raw data, automatic tokens and coins forwardings, callback functionalities, and much more.
 *
 * The version of the OpenAPI document: 2.0.0
 * Contact: developers@cryptoapis.io
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method `delete_blockchain_event_subscription`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteBlockchainEventSubscriptionError {
    Status400(crate::models::InvalidPagination),
    Status401(crate::models::InvalidApiKey),
    Status402(crate::models::InsufficientCredits),
    Status403(crate::models::FeatureMainnetsNotAllowedForPlan),
    Status404(crate::models::ResourceNotFound),
    Status409(crate::models::InvalidData),
    Status415(crate::models::UnsupportedMediaType),
    Status422(crate::models::InvalidRequestBodyStructure),
    Status429(crate::models::RequestLimitReached),
    Status500(crate::models::UnexpectedServerError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `list_blockchain_events_subscriptions`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListBlockchainEventsSubscriptionsError {
    Status400(crate::models::InvalidPagination),
    Status401(crate::models::InvalidApiKey),
    Status402(crate::models::InsufficientCredits),
    Status403(crate::models::FeatureMainnetsNotAllowedForPlan),
    Status409(crate::models::InvalidData),
    Status415(crate::models::UnsupportedMediaType),
    Status422(crate::models::InvalidRequestBodyStructure),
    Status429(crate::models::RequestLimitReached),
    Status500(crate::models::UnexpectedServerError),
    UnknownValue(serde_json::Value),
}


/// Through this endpoint the customer can delete blockchain event subscriptions they have by attributes `referenceId` and `network`.    Currently Crypto APIs 2.0 offers certain Blockchain event endpoints which allow the user to subscribe for one/a few/all and receive callback notifications when the specific event occurs.    {note}To have an operational callback subscription, you need to first verify a domain for the Callback URL. Please see more information on Callbacks [here](https://developers.cryptoapis.io/technical-documentation/general-information/callbacks#callback-url).{/note}    {warning}Crypto APIs will notify the user **only when** the event occurs. There are cases when the specific event doesn't happen at all, or takes a long time to do so. A callback notification **will not** be sent if the event does not or cannot occur, or will take long time to occur.{/warning}
pub async fn delete_blockchain_event_subscription(configuration: &configuration::Configuration, blockchain: &str, network: &str, reference_id: &str, context: Option<&str>) -> Result<crate::models::DeleteBlockchainEventSubscriptionR, Error<DeleteBlockchainEventSubscriptionError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/blockchain-events/{blockchain}/{network}/subscriptions/{referenceId}", configuration.base_path, blockchain=crate::apis::urlencode(blockchain), network=crate::apis::urlencode(network), referenceId=crate::apis::urlencode(reference_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = context {
        local_var_req_builder = local_var_req_builder.query(&[("context", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("x-api-key", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DeleteBlockchainEventSubscriptionError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Through this endpoint the customer can obtain a list of their callback subscriptions for the available Blockchain events.    Currently Crypto APIs 2.0 offers certain Blockchain event endpoints which allow the user to subscribe for one/a few/all and receive callback notifications when the specific event occurs.    {note}To have an operational callback subscription, you need to first verify a domain for the Callback URL. Please see more information on Callbacks [here](https://developers.cryptoapis.io/technical-documentation/general-information/callbacks#callback-url).{/note}
pub async fn list_blockchain_events_subscriptions(configuration: &configuration::Configuration, blockchain: &str, network: &str, context: Option<&str>, limit: Option<i32>, offset: Option<i32>) -> Result<crate::models::ListBlockchainEventsSubscriptionsR, Error<ListBlockchainEventsSubscriptionsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/blockchain-events/{blockchain}/{network}/subscriptions", configuration.base_path, blockchain=crate::apis::urlencode(blockchain), network=crate::apis::urlencode(network));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = context {
        local_var_req_builder = local_var_req_builder.query(&[("context", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = offset {
        local_var_req_builder = local_var_req_builder.query(&[("offset", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("x-api-key", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ListBlockchainEventsSubscriptionsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

