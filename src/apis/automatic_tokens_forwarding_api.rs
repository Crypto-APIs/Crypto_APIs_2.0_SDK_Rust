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


/// struct for typed errors of method [`add_tokens_to_existing_from_address`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddTokensToExistingFromAddressError {
    Status400(crate::models::InvalidPagination),
    Status401(crate::models::InvalidApiKey),
    Status402(crate::models::InsufficientCredits),
    Status403(crate::models::TokensForwardingAutomationsLimitReached),
    Status404(crate::models::ResourceNotFound),
    Status409(crate::models::InvalidData),
    Status415(crate::models::UnsupportedMediaType),
    Status422(crate::models::InvalidRequestBodyStructure),
    Status429(crate::models::RequestLimitReached),
    Status500(crate::models::UnexpectedServerError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_automatic_tokens_forwarding`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateAutomaticTokensForwardingError {
    Status400(crate::models::InvalidPagination),
    Status401(crate::models::InvalidApiKey),
    Status402(crate::models::InsufficientCredits),
    Status403(crate::models::TokensForwardingAutomationsLimitReached),
    Status404(crate::models::ResourceNotFound),
    Status409(crate::models::InvalidData),
    Status415(crate::models::UnsupportedMediaType),
    Status422(crate::models::InvalidRequestBodyStructure),
    Status429(crate::models::RequestLimitReached),
    Status500(crate::models::UnexpectedServerError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_automatic_tokens_forwarding`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteAutomaticTokensForwardingError {
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

/// struct for typed errors of method [`get_fee_address_details`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetFeeAddressDetailsError {
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

/// struct for typed errors of method [`list_tokens_forwarding_automations`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListTokensForwardingAutomationsError {
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


/// Through this endpoint customers can add **Automatic Tokens forwardings** to an already existing `fromAddress`. Unlike the \"Create Automatic Tokens Forwarding\" endpoint, where the `fromAddress` is generated each time, with this endpoint customers can add an automation from another token to one and the same `fromAddress`.    The `fromAddress` can be used as a deposit address. Any funds received by this address will be automatically forwarded to `toAddress` based on what the customer has set for the automation. The  `toAddress` is essentially the main address and destination for the automatic tokens forwarding.    There is also a `minimumTransferAmount` which only when reached will then trigger the forwarding. Through this the customer can save from fees.    Moreover, `feePriority` can be also set,  which defines how quickly to move the tokens once they are received. The higher priority, the larger the fee will be. It can be \"SLOW\", \"STANDARD\" or \"FAST\".    For this automatic forwarding the customer can set a callback subscription.    {warning}The subscription will work for all incoming transactions until it is deleted. There is no need to do that for every transaction.{/warning}
pub async fn add_tokens_to_existing_from_address(configuration: &configuration::Configuration, blockchain: &str, network: &str, context: Option<&str>, add_tokens_to_existing_from_address_rb: Option<crate::models::AddTokensToExistingFromAddressRb>) -> Result<crate::models::AddTokensToExistingFromAddressR, Error<AddTokensToExistingFromAddressError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/blockchain-automations/{blockchain}/{network}/tokens-forwarding/automations/add-token", local_var_configuration.base_path, blockchain=crate::apis::urlencode(blockchain), network=crate::apis::urlencode(network));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = context {
        local_var_req_builder = local_var_req_builder.query(&[("context", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("x-api-key", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&add_tokens_to_existing_from_address_rb);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AddTokensToExistingFromAddressError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Through this endpoint customers can set up an automatic forwarding function specifically for tokens (**not** coins). They can have a `toAddress` which is essentially the main address and the destination for the automatic tokens forwarding.     There is also a `minimumTransferAmount` which only when reached will then trigger the forwarding. Through this the customer can save from fees.    Moreover, `feePriority` can be also set,  which defines how quickly to move the tokens once they are received. The higher priority, the larger the fee will be. It can be \"SLOW\", \"STANDARD\" or \"FAST\".    The response of this endpoint contains an attribute `fromAddress` which can be used as a deposit address. Any funds received by this address will be automatically forwarded to `toAddress` based on what the customer has set for the automation.    For this automatic forwarding the customer can set a callback subscription.    {warning}The subscription will work for all incoming transactions until it is deleted. There is no need to do that for every transaction.{/warning}    {note}This endpoint generates a new `fromAddress` each time.{/note}
pub async fn create_automatic_tokens_forwarding(configuration: &configuration::Configuration, blockchain: &str, network: &str, context: Option<&str>, create_automatic_tokens_forwarding_rb: Option<crate::models::CreateAutomaticTokensForwardingRb>) -> Result<crate::models::CreateAutomaticTokensForwardingR, Error<CreateAutomaticTokensForwardingError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/blockchain-automations/{blockchain}/{network}/tokens-forwarding/automations", local_var_configuration.base_path, blockchain=crate::apis::urlencode(blockchain), network=crate::apis::urlencode(network));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = context {
        local_var_req_builder = local_var_req_builder.query(&[("context", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("x-api-key", local_var_value);
    };
    local_var_req_builder = local_var_req_builder.json(&create_automatic_tokens_forwarding_rb);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateAutomaticTokensForwardingError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Through this endpoint customers can delete a forwarding function they have set for **tokens** (**not** coins).    By setting a `fromAddress` and a `toAddress`, and specifying the amount, tokens can be transferred between addresses.     A `feePriority` will be returned which represents the fee priority of the automation whether it is \"SLOW\", \"STANDARD\" OR \"FAST\".    {warning}The subscription will work for all incoming transactions until it is deleted. There is no need to do that for every transaction.{/warning}
pub async fn delete_automatic_tokens_forwarding(configuration: &configuration::Configuration, blockchain: &str, network: &str, reference_id: &str, context: Option<&str>) -> Result<crate::models::DeleteAutomaticTokensForwardingR, Error<DeleteAutomaticTokensForwardingError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/blockchain-automations/{blockchain}/{network}/tokens-forwarding/automations/{referenceId}", local_var_configuration.base_path, blockchain=crate::apis::urlencode(blockchain), network=crate::apis::urlencode(network), referenceId=crate::apis::urlencode(reference_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = context {
        local_var_req_builder = local_var_req_builder.query(&[("context", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
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
        let local_var_entity: Option<DeleteAutomaticTokensForwardingError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Through this endpoint customers can obtain details about a fee address. Only one fee address per currency per network for a user's account can be set no matter how many tokens or subscriptions they have or want to automatically forward.
pub async fn get_fee_address_details(configuration: &configuration::Configuration, blockchain: &str, network: &str, context: Option<&str>) -> Result<crate::models::GetFeeAddressDetailsR, Error<GetFeeAddressDetailsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/blockchain-automations/{blockchain}/{network}/tokens-forwarding/fee-addresses", local_var_configuration.base_path, blockchain=crate::apis::urlencode(blockchain), network=crate::apis::urlencode(network));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = context {
        local_var_req_builder = local_var_req_builder.query(&[("context", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
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
        let local_var_entity: Option<GetFeeAddressDetailsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Through this endpoint customers can list all of their **tokens** forwarding automations (**not** coins).    Customers can set up automatic forwarding functions for tokens by setting a `fromAddress` and a `toAddress`, and specifying the amount that can be transferred between addresses.     A `feePriority` will be returned which represents the fee priority of the automation whether it is \"SLOW\", \"STANDARD\" OR \"FAST\".     {warning}The subscription will work for all transactions until it is deleted. There is no need to do that for every transaction.{/warning}
pub async fn list_tokens_forwarding_automations(configuration: &configuration::Configuration, blockchain: &str, network: &str, context: Option<&str>, limit: Option<i32>, offset: Option<i32>) -> Result<crate::models::ListTokensForwardingAutomationsR, Error<ListTokensForwardingAutomationsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/blockchain-automations/{blockchain}/{network}/tokens-forwarding/automations", local_var_configuration.base_path, blockchain=crate::apis::urlencode(blockchain), network=crate::apis::urlencode(network));
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
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
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
        let local_var_entity: Option<ListTokensForwardingAutomationsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

