/*
 * CryptoAPIs
 *
 * Crypto APIs is a complex and innovative infrastructure layer that radically simplifies the development of any Blockchain and Crypto related applications. Organized around REST, Crypto APIs can assist both novice Bitcoin/Ethereum enthusiasts and crypto experts with the development of their blockchain applications. Crypto APIs provides unified endpoints and data, raw data, automatic tokens and coins forwardings, callback functionalities, and much more.
 *
 * The version of the OpenAPI document: 2021-03-20
 * Contact: developers@cryptoapis.io
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method `get_latest_mined_xrp__ripple_block`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLatestMinedXrpRippleBlockError {
    Status400(crate::models::InlineResponse40036),
    Status401(crate::models::InlineResponse40136),
    Status402(crate::models::InlineResponse402),
    Status403(crate::models::InlineResponse40336),
    Status404(crate::models::InlineResponse4041),
    Status409(crate::models::InlineResponse409),
    Status415(crate::models::InlineResponse415),
    Status422(crate::models::InlineResponse422),
    Status429(crate::models::InlineResponse429),
    Status500(crate::models::InlineResponse500),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_xrp__ripple_address_details`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetXrpRippleAddressDetailsError {
    Status400(crate::models::InlineResponse4003),
    Status401(crate::models::InlineResponse4013),
    Status402(crate::models::InlineResponse402),
    Status403(crate::models::InlineResponse4033),
    Status409(crate::models::InlineResponse409),
    Status415(crate::models::InlineResponse415),
    Status422(crate::models::InlineResponse422),
    Status429(crate::models::InlineResponse429),
    Status500(crate::models::InlineResponse500),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_xrp__ripple_block_details_by_block_hash`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetXrpRippleBlockDetailsByBlockHashError {
    Status400(crate::models::InlineResponse40028),
    Status401(crate::models::InlineResponse40128),
    Status402(crate::models::InlineResponse402),
    Status403(crate::models::InlineResponse40328),
    Status404(crate::models::InlineResponse4041),
    Status409(crate::models::InlineResponse409),
    Status415(crate::models::InlineResponse415),
    Status422(crate::models::InlineResponse422),
    Status429(crate::models::InlineResponse429),
    Status500(crate::models::InlineResponse500),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_xrp__ripple_block_details_by_block_height`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetXrpRippleBlockDetailsByBlockHeightError {
    Status400(crate::models::InlineResponse40025),
    Status401(crate::models::InlineResponse40125),
    Status402(crate::models::InlineResponse402),
    Status403(crate::models::InlineResponse40325),
    Status404(crate::models::InlineResponse4041),
    Status409(crate::models::InlineResponse409),
    Status415(crate::models::InlineResponse415),
    Status422(crate::models::InlineResponse422),
    Status429(crate::models::InlineResponse429),
    Status500(crate::models::InlineResponse500),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_xrp__ripple_transaction_details_by_transaction_id`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetXrpRippleTransactionDetailsByTransactionIdError {
    Status400(crate::models::InlineResponse4006),
    Status401(crate::models::InlineResponse4016),
    Status402(crate::models::InlineResponse402),
    Status403(crate::models::InlineResponse4036),
    Status404(crate::models::InlineResponse4041),
    Status409(crate::models::InlineResponse409),
    Status415(crate::models::InlineResponse415),
    Status422(crate::models::InlineResponse422),
    Status429(crate::models::InlineResponse429),
    Status500(crate::models::InlineResponse500),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `list_xrp__ripple_transactions_by_address`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListXrpRippleTransactionsByAddressError {
    Status400(crate::models::InlineResponse40011),
    Status401(crate::models::InlineResponse40111),
    Status402(crate::models::InlineResponse402),
    Status403(crate::models::InlineResponse40311),
    Status409(crate::models::InlineResponse409),
    Status415(crate::models::InlineResponse415),
    Status422(crate::models::InlineResponse422),
    Status429(crate::models::InlineResponse429),
    Status500(crate::models::InlineResponse500),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `list_xrp__ripple_transactions_by_address_and_time_range`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListXrpRippleTransactionsByAddressAndTimeRangeError {
    Status400(crate::models::InlineResponse40015),
    Status401(crate::models::InlineResponse40115),
    Status402(crate::models::InlineResponse402),
    Status403(crate::models::InlineResponse40315),
    Status409(crate::models::InlineResponse409),
    Status415(crate::models::InlineResponse415),
    Status422(crate::models::InlineResponse422),
    Status429(crate::models::InlineResponse429),
    Status500(crate::models::InlineResponse500),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `list_xrp__ripple_transactions_by_block_hash`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListXrpRippleTransactionsByBlockHashError {
    Status400(crate::models::InlineResponse40017),
    Status401(crate::models::InlineResponse40117),
    Status402(crate::models::InlineResponse402),
    Status403(crate::models::InlineResponse40317),
    Status409(crate::models::InlineResponse409),
    Status415(crate::models::InlineResponse415),
    Status422(crate::models::InlineResponse422),
    Status429(crate::models::InlineResponse429),
    Status500(crate::models::InlineResponse500),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `list_xrp__ripple_transactions_by_block_height`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListXrpRippleTransactionsByBlockHeightError {
    Status400(crate::models::InlineResponse40021),
    Status401(crate::models::InlineResponse40121),
    Status402(crate::models::InlineResponse402),
    Status403(crate::models::InlineResponse40321),
    Status409(crate::models::InlineResponse409),
    Status415(crate::models::InlineResponse415),
    Status422(crate::models::InlineResponse422),
    Status429(crate::models::InlineResponse429),
    Status500(crate::models::InlineResponse500),
    UnknownValue(serde_json::Value),
}


/// Through this endpoint customers can fetch the last mined XRP block in the blockchain, along with its details. These could include the hash of the specific, the previous and the next block, its transactions count, its height, etc.     Since XRP is a different blockchain than Bitcoin and Ethereum, it isn't unified.
pub async fn get_latest_mined_xrp__ripple_block(configuration: &configuration::Configuration, network: &str, context: Option<&str>) -> Result<crate::models::GetLatestMinedXrpRippleBlockR, Error<GetLatestMinedXrpRippleBlockError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/blockchain-data/xrp-specific/{network}/blocks/last", configuration.base_path, network=crate::apis::urlencode(network));
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
        let local_var_entity: Option<GetLatestMinedXrpRippleBlockError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Through this endpoint the customer can receive basic information about a given XRP address based on confirmed/synced blocks only. In the case where there are any incoming or outgoing **unconfirmed** transactions for the specific address, they **will not** be counted or calculated here.    Since XRP is a different blockchain than Bitcoin and Ethereum, it isn't unified.
pub async fn get_xrp__ripple_address_details(configuration: &configuration::Configuration, network: &str, address: &str, context: Option<&str>) -> Result<crate::models::GetXrpRippleAddressDetailsR, Error<GetXrpRippleAddressDetailsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/blockchain-data/xrp-specific/{network}/addresses/{address}", configuration.base_path, network=crate::apis::urlencode(network), address=crate::apis::urlencode(address));
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
        let local_var_entity: Option<GetXrpRippleAddressDetailsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Through this endpoint customers can obtain basic information about a given XRP block (a block on the XRP blockchain), specifically by using the `hash` parameter. These block details could include the hash of the specific, the previous and the next block, the number of included transactions, etc.     Since XRP is a different blockchain than Bitcoin and Ethereum, it isn't unified.
pub async fn get_xrp__ripple_block_details_by_block_hash(configuration: &configuration::Configuration, network: &str, block_hash: &str, context: Option<&str>) -> Result<crate::models::GetXrpRippleBlockDetailsByBlockHashR, Error<GetXrpRippleBlockDetailsByBlockHashError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/blockchain-data/xrp-specific/{network}/blocks/hash/{blockHash}", configuration.base_path, network=crate::apis::urlencode(network), blockHash=crate::apis::urlencode(block_hash));
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
        let local_var_entity: Option<GetXrpRippleBlockDetailsByBlockHashError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Through this endpoint customers can obtain basic information about a given XRP block (a block on the XRP blockchain), specifically by using the `height` parameter. These block details could include the hash of the specific, the previous and the next block, its transactions count, etc.    Since XRP is a different blockchain than Bitcoin and Ethereum, it isn't unified.
pub async fn get_xrp__ripple_block_details_by_block_height(configuration: &configuration::Configuration, network: &str, block_height: &str, context: Option<&str>) -> Result<crate::models::GetXrpRippleBlockDetailsByBlockHeightR, Error<GetXrpRippleBlockDetailsByBlockHeightError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/blockchain-data/xrp-specific/{network}/blocks/height/{blockHeight}", configuration.base_path, network=crate::apis::urlencode(network), blockHeight=crate::apis::urlencode(block_height));
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
        let local_var_entity: Option<GetXrpRippleBlockDetailsByBlockHeightError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Through this endpoint customers can obtain details about a XRP transaction by the transaction's unique identifier.     Since XRP is a different blockchain than Bitcoin and Ethereum, it isn't unified.
pub async fn get_xrp__ripple_transaction_details_by_transaction_id(configuration: &configuration::Configuration, network: &str, transaction_hash: &str, context: Option<&str>) -> Result<crate::models::GetXrpRippleTransactionDetailsByTransactionIdr, Error<GetXrpRippleTransactionDetailsByTransactionIdError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/blockchain-data/xrp-specific/{network}/transactions/{transactionHash}", configuration.base_path, network=crate::apis::urlencode(network), transactionHash=crate::apis::urlencode(transaction_hash));
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
        let local_var_entity: Option<GetXrpRippleTransactionDetailsByTransactionIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// This endpoint will list XRP transactions by a attribute `address`. The transactions listed will detail additional information such as hash, height, time of creation in Unix timestamp, etc.    Since XRP is a different blockchain than Bitcoin and Ethereum, it isn't unified.
pub async fn list_xrp__ripple_transactions_by_address(configuration: &configuration::Configuration, network: &str, address: &str, context: Option<&str>, limit: Option<i64>, offset: Option<i64>, transaction_type: Option<&str>) -> Result<crate::models::ListXrpRippleTransactionsByAddressR, Error<ListXrpRippleTransactionsByAddressError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/blockchain-data/xrp-specific/{network}/addresses/{address}/transactions", configuration.base_path, network=crate::apis::urlencode(network), address=crate::apis::urlencode(address));
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
        let local_var_entity: Option<ListXrpRippleTransactionsByAddressError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Тhis endpoint lists XRP transactions by the attribute `address` and the query parameters `fromTimestamp` and `toTimestamp`  which gives customers the opportunity to filter the results by a specified time period.
pub async fn list_xrp__ripple_transactions_by_address_and_time_range(configuration: &configuration::Configuration, network: &str, address: &str, from_timestamp: i32, to_timestamp: i32, context: Option<&str>, limit: Option<i64>, offset: Option<i64>, transaction_type: Option<&str>) -> Result<crate::models::ListXrpRippleTransactionsByAddressAndTimeRangeR, Error<ListXrpRippleTransactionsByAddressAndTimeRangeError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/blockchain-data/xrp-specific/{network}/addresses/{address}/transactions-by-time-range", configuration.base_path, network=crate::apis::urlencode(network), address=crate::apis::urlencode(address));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = context {
        local_var_req_builder = local_var_req_builder.query(&[("context", &local_var_str.to_string())]);
    }
    local_var_req_builder = local_var_req_builder.query(&[("fromTimestamp", &from_timestamp.to_string())]);
    if let Some(ref local_var_str) = limit {
        local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = offset {
        local_var_req_builder = local_var_req_builder.query(&[("offset", &local_var_str.to_string())]);
    }
    local_var_req_builder = local_var_req_builder.query(&[("toTimestamp", &to_timestamp.to_string())]);
    if let Some(ref local_var_str) = transaction_type {
        local_var_req_builder = local_var_req_builder.query(&[("transactionType", &local_var_str.to_string())]);
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
        let local_var_entity: Option<ListXrpRippleTransactionsByAddressAndTimeRangeError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// This endpoint will list transactions by an attribute `blockHash`. The transactions listed will detail additional information such as hash, addresses, time of creation in Unix timestamp, etc.    Since XRP is a different blockchain than Bitcoin and Ethereum, it isn't unified.
pub async fn list_xrp__ripple_transactions_by_block_hash(configuration: &configuration::Configuration, network: &str, block_hash: &str, context: Option<&str>, limit: Option<i64>, offset: Option<i64>) -> Result<crate::models::ListXrpRippleTransactionsByBlockHashR, Error<ListXrpRippleTransactionsByBlockHashError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/blockchain-data/xrp-specific/{network}/blocks/hash/{blockHash}/transactions", configuration.base_path, network=crate::apis::urlencode(network), blockHash=crate::apis::urlencode(block_hash));
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
        let local_var_entity: Option<ListXrpRippleTransactionsByBlockHashError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// This endpoint will list transactions by an attribute `blockHeight`. The transactions listed will detail additional information such as hash, addresses, time of creation in Unix timestamp, etc.    Since XRP is a different blockchain than Bitcoin and Ethereum, it isn't unified.
pub async fn list_xrp__ripple_transactions_by_block_height(configuration: &configuration::Configuration, network: &str, block_height: i64, context: Option<&str>, limit: Option<i64>, offset: Option<i64>) -> Result<crate::models::ListXrpRippleTransactionsByBlockHeightR, Error<ListXrpRippleTransactionsByBlockHeightError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/blockchain-data/xrp-specific/{network}/blocks/height/{blockHeight}/transactions", configuration.base_path, network=crate::apis::urlencode(network), blockHeight=block_height);
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
        let local_var_entity: Option<ListXrpRippleTransactionsByBlockHeightError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

