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


/// struct for typed errors of method `get_token_details_by_contract_address`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetTokenDetailsByContractAddressError {
    Status400(crate::models::InlineResponse40057),
    Status401(crate::models::InlineResponse40157),
    Status402(crate::models::InlineResponse402),
    Status403(crate::models::InlineResponse40357),
    Status409(crate::models::InlineResponse409),
    Status415(crate::models::InlineResponse415),
    Status422(crate::models::InlineResponse422),
    Status429(crate::models::InlineResponse429),
    Status500(crate::models::InlineResponse500),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `list_confirmed_tokens_transfers_by_address`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListConfirmedTokensTransfersByAddressError {
    Status400(crate::models::InlineResponse40055),
    Status401(crate::models::InlineResponse40155),
    Status402(crate::models::InlineResponse402),
    Status403(crate::models::InlineResponse40355),
    Status409(crate::models::InlineResponse409),
    Status415(crate::models::InlineResponse415),
    Status422(crate::models::InlineResponse422),
    Status429(crate::models::InlineResponse429),
    Status500(crate::models::InlineResponse500),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `list_tokens_by_address`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListTokensByAddressError {
    Status400(crate::models::InlineResponse40056),
    Status401(crate::models::InlineResponse40156),
    Status402(crate::models::InlineResponse402),
    Status403(crate::models::InlineResponse40356),
    Status409(crate::models::InlineResponse409),
    Status415(crate::models::InlineResponse415),
    Status422(crate::models::InlineResponse422),
    Status429(crate::models::InlineResponse429),
    Status500(crate::models::InlineResponse500),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `list_tokens_transfers_by_transaction_hash`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListTokensTransfersByTransactionHashError {
    Status400(crate::models::InlineResponse40054),
    Status401(crate::models::InlineResponse40154),
    Status402(crate::models::InlineResponse402),
    Status403(crate::models::InlineResponse40354),
    Status409(crate::models::InlineResponse409),
    Status415(crate::models::InlineResponse415),
    Status422(crate::models::InlineResponse422),
    Status429(crate::models::InlineResponse429),
    Status500(crate::models::InlineResponse500),
    UnknownValue(serde_json::Value),
}


/// Though this endpoint customers can obtain information about token details. This can be done by providing the `contact address` parameter.    {note}This address is **not** the same as the smart contract creator address.{/note}
pub async fn get_token_details_by_contract_address(configuration: &configuration::Configuration, blockchain: &str, network: &str, contract_address: &str, context: Option<&str>) -> Result<crate::models::GetTokenDetailsByContractAddressR, Error<GetTokenDetailsByContractAddressError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/blockchain-data/{blockchain}/{network}/addresses/{contractAddress}/contract", configuration.base_path, blockchain=crate::apis::urlencode(blockchain), network=crate::apis::urlencode(network), contractAddress=crate::apis::urlencode(contract_address));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

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
        let local_var_entity: Option<GetTokenDetailsByContractAddressError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Through this endpoint customers can obtain a list with **confirmed** token transfers by the `address` attribute. Token transfers may include information such as addresses of the sender and recipient, token name, token symbol, etc.    {note}This refers only to transfers done for **confirmed tokens** not coins.{/note}
pub async fn list_confirmed_tokens_transfers_by_address(configuration: &configuration::Configuration, blockchain: &str, network: &str, address: &str, context: Option<&str>, limit: Option<i32>, offset: Option<i32>) -> Result<crate::models::ListConfirmedTokensTransfersByAddressR, Error<ListConfirmedTokensTransfersByAddressError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/blockchain-data/{blockchain}/{network}/addresses/{address}/tokens-transfers", configuration.base_path, blockchain=crate::apis::urlencode(blockchain), network=crate::apis::urlencode(network), address=crate::apis::urlencode(address));
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
        let local_var_entity: Option<ListConfirmedTokensTransfersByAddressError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Through this endpoint customers can obtain token data by providing an attribute - `address`.  The information that can be returned can include the contract address, the token symbol, type and balance.
pub async fn list_tokens_by_address(configuration: &configuration::Configuration, blockchain: &str, network: &str, address: &str, context: Option<&str>, limit: Option<i32>, offset: Option<i32>) -> Result<crate::models::ListTokensByAddressR, Error<ListTokensByAddressError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/blockchain-data/{blockchain}/{network}/addresses/{address}/tokens", configuration.base_path, blockchain=crate::apis::urlencode(blockchain), network=crate::apis::urlencode(network), address=crate::apis::urlencode(address));
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
        let local_var_entity: Option<ListTokensByAddressError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Through this endpoint customers can obtain a list with token transfers by the `transactionHash` attribute. Token transfers may include information such as addresses of the sender and recipient, token name, token symbol, etc.    {note}This refers only to transfers done for **tokens** not coins.{/note}
pub async fn list_tokens_transfers_by_transaction_hash(configuration: &configuration::Configuration, blockchain: &str, network: &str, transaction_hash: &str, context: Option<&str>, limit: Option<i32>, offset: Option<i32>) -> Result<crate::models::ListTokensTransfersByTransactionHashR, Error<ListTokensTransfersByTransactionHashError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/blockchain-data/{blockchain}/{network}/transactions/{transactionHash}/tokens-transfers", configuration.base_path, blockchain=crate::apis::urlencode(blockchain), network=crate::apis::urlencode(network), transactionHash=crate::apis::urlencode(transaction_hash));
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
        let local_var_entity: Option<ListTokensTransfersByTransactionHashError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

