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


/// struct for typed errors of method [`get_latest_mined_xrp__ripple_block`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLatestMinedXrpRippleBlockError {
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

/// struct for typed errors of method [`get_xrp__ripple_address_details`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetXrpRippleAddressDetailsError {
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

/// struct for typed errors of method [`get_xrp__ripple_block_details_by_block_hash`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetXrpRippleBlockDetailsByBlockHashError {
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

/// struct for typed errors of method [`get_xrp__ripple_block_details_by_block_height`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetXrpRippleBlockDetailsByBlockHeightError {
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

/// struct for typed errors of method [`get_xrp__ripple_transaction_details_by_transaction_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetXrpRippleTransactionDetailsByTransactionIdError {
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

/// struct for typed errors of method [`list_xrp__ripple_transactions_by_address`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListXrpRippleTransactionsByAddressError {
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

/// struct for typed errors of method [`list_xrp__ripple_transactions_by_block_hash`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListXrpRippleTransactionsByBlockHashError {
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

/// struct for typed errors of method [`list_xrp__ripple_transactions_by_block_height`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListXrpRippleTransactionsByBlockHeightError {
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


/// Through this endpoint customers can fetch the last mined XRP block in the blockchain, along with its details. These could include the hash of the specific, the previous and the next block, its transactions count, its height, etc.     Since XRP is a different blockchain than Bitcoin and Ethereum, it isn't unified.
pub async fn get_latest_mined_xrp__ripple_block(configuration: &configuration::Configuration, network: &str, context: Option<&str>) -> Result<crate::models::GetLatestMinedXrpRippleBlockR, Error<GetLatestMinedXrpRippleBlockError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/blockchain-data/xrp-specific/{network}/blocks/last", local_var_configuration.base_path, network=crate::apis::urlencode(network));
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
        let local_var_entity: Option<GetLatestMinedXrpRippleBlockError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Through this endpoint the customer can receive basic information about a given XRP address based on confirmed/synced blocks only. In the case where there are any incoming or outgoing **unconfirmed** transactions for the specific address, they **will not** be counted or calculated here.    Since XRP is a different blockchain than Bitcoin and Ethereum, it isn't unified.
pub async fn get_xrp__ripple_address_details(configuration: &configuration::Configuration, network: &str, address: &str, context: Option<&str>) -> Result<crate::models::GetXrpRippleAddressDetailsR, Error<GetXrpRippleAddressDetailsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/blockchain-data/xrp-specific/{network}/addresses/{address}", local_var_configuration.base_path, network=crate::apis::urlencode(network), address=crate::apis::urlencode(address));
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
        let local_var_entity: Option<GetXrpRippleAddressDetailsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Through this endpoint customers can obtain basic information about a given XRP block (a block on the XRP blockchain), specifically by using the `hash` parameter. These block details could include the hash of the specific, the previous and the next block, the number of included transactions, etc.     Since XRP is a different blockchain than Bitcoin and Ethereum, it isn't unified.
pub async fn get_xrp__ripple_block_details_by_block_hash(configuration: &configuration::Configuration, network: &str, block_hash: &str, context: Option<&str>) -> Result<crate::models::GetXrpRippleBlockDetailsByBlockHashR, Error<GetXrpRippleBlockDetailsByBlockHashError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/blockchain-data/xrp-specific/{network}/blocks/hash/{blockHash}", local_var_configuration.base_path, network=crate::apis::urlencode(network), blockHash=crate::apis::urlencode(block_hash));
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
        let local_var_entity: Option<GetXrpRippleBlockDetailsByBlockHashError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Through this endpoint customers can obtain basic information about a given XRP block (a block on the XRP blockchain), specifically by using the `height` parameter. These block details could include the hash of the specific, the previous and the next block, its transactions count, etc.    Since XRP is a different blockchain than Bitcoin and Ethereum, it isn't unified.
pub async fn get_xrp__ripple_block_details_by_block_height(configuration: &configuration::Configuration, network: &str, block_height: &str, context: Option<&str>) -> Result<crate::models::GetXrpRippleBlockDetailsByBlockHeightR, Error<GetXrpRippleBlockDetailsByBlockHeightError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/blockchain-data/xrp-specific/{network}/blocks/height/{blockHeight}", local_var_configuration.base_path, network=crate::apis::urlencode(network), blockHeight=crate::apis::urlencode(block_height));
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
        let local_var_entity: Option<GetXrpRippleBlockDetailsByBlockHeightError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Through this endpoint customers can obtain details about a XRP transaction by the transaction's unique identifier.     Since XRP is a different blockchain than Bitcoin and Ethereum, it isn't unified.
pub async fn get_xrp__ripple_transaction_details_by_transaction_id(configuration: &configuration::Configuration, network: &str, transaction_hash: &str, context: Option<&str>) -> Result<crate::models::GetXrpRippleTransactionDetailsByTransactionIdr, Error<GetXrpRippleTransactionDetailsByTransactionIdError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/blockchain-data/xrp-specific/{network}/transactions/{transactionHash}", local_var_configuration.base_path, network=crate::apis::urlencode(network), transactionHash=crate::apis::urlencode(transaction_hash));
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
        let local_var_entity: Option<GetXrpRippleTransactionDetailsByTransactionIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// This endpoint will list XRP transactions by a attribute `address`. The transactions listed will detail additional information such as hash, height, time of creation in Unix timestamp, etc.    Since XRP is a different blockchain than Bitcoin and Ethereum, it isn't unified.
pub async fn list_xrp__ripple_transactions_by_address(configuration: &configuration::Configuration, network: &str, address: &str, context: Option<&str>, limit: Option<i32>, offset: Option<i32>, transaction_type: Option<&str>) -> Result<crate::models::ListXrpRippleTransactionsByAddressR, Error<ListXrpRippleTransactionsByAddressError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/blockchain-data/xrp-specific/{network}/addresses/{address}/transactions", local_var_configuration.base_path, network=crate::apis::urlencode(network), address=crate::apis::urlencode(address));
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
    if let Some(ref local_var_str) = transaction_type {
        local_var_req_builder = local_var_req_builder.query(&[("transactionType", &local_var_str.to_string())]);
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
        let local_var_entity: Option<ListXrpRippleTransactionsByAddressError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// This endpoint will list transactions by an attribute `blockHash`. The transactions listed will detail additional information such as hash, addresses, time of creation in Unix timestamp, etc.    Since XRP is a different blockchain than Bitcoin and Ethereum, it isn't unified.
pub async fn list_xrp__ripple_transactions_by_block_hash(configuration: &configuration::Configuration, network: &str, block_hash: &str, context: Option<&str>, limit: Option<i32>, offset: Option<i32>) -> Result<crate::models::ListXrpRippleTransactionsByBlockHashR, Error<ListXrpRippleTransactionsByBlockHashError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/blockchain-data/xrp-specific/{network}/blocks/hash/{blockHash}/transactions", local_var_configuration.base_path, network=crate::apis::urlencode(network), blockHash=crate::apis::urlencode(block_hash));
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
        let local_var_entity: Option<ListXrpRippleTransactionsByBlockHashError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// This endpoint will list transactions by an attribute `blockHeight`. The transactions listed will detail additional information such as hash, addresses, time of creation in Unix timestamp, etc.    Since XRP is a different blockchain than Bitcoin and Ethereum, it isn't unified.
pub async fn list_xrp__ripple_transactions_by_block_height(configuration: &configuration::Configuration, network: &str, block_height: i32, context: Option<&str>, limit: Option<i32>, offset: Option<i32>) -> Result<crate::models::ListXrpRippleTransactionsByBlockHeightR, Error<ListXrpRippleTransactionsByBlockHeightError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/blockchain-data/xrp-specific/{network}/blocks/height/{blockHeight}/transactions", local_var_configuration.base_path, network=crate::apis::urlencode(network), blockHeight=block_height);
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
        let local_var_entity: Option<ListXrpRippleTransactionsByBlockHeightError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

