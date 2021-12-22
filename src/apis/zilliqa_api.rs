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


/// struct for typed errors of method `get_latest_mined_zilliqa_block`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLatestMinedZilliqaBlockError {
    Status400(crate::models::InlineResponse40040),
    Status401(crate::models::InlineResponse40140),
    Status402(crate::models::InlineResponse402),
    Status403(crate::models::InlineResponse40340),
    Status404(crate::models::InlineResponse4042),
    Status409(crate::models::InlineResponse409),
    Status415(crate::models::InlineResponse415),
    Status422(crate::models::InlineResponse422),
    Status429(crate::models::InlineResponse429),
    Status500(crate::models::InlineResponse500),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_zilliqa_address_details`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetZilliqaAddressDetailsError {
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

/// struct for typed errors of method `get_zilliqa_block_details_by_block_hash`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetZilliqaBlockDetailsByBlockHashError {
    Status400(crate::models::InlineResponse40033),
    Status401(crate::models::InlineResponse40133),
    Status402(crate::models::InlineResponse402),
    Status403(crate::models::InlineResponse40333),
    Status404(crate::models::InlineResponse4042),
    Status409(crate::models::InlineResponse409),
    Status415(crate::models::InlineResponse415),
    Status422(crate::models::InlineResponse422),
    Status429(crate::models::InlineResponse429),
    Status500(crate::models::InlineResponse500),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_zilliqa_block_details_by_block_height`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetZilliqaBlockDetailsByBlockHeightError {
    Status400(crate::models::InlineResponse40029),
    Status401(crate::models::InlineResponse40129),
    Status402(crate::models::InlineResponse402),
    Status403(crate::models::InlineResponse40329),
    Status404(crate::models::InlineResponse4042),
    Status409(crate::models::InlineResponse409),
    Status415(crate::models::InlineResponse415),
    Status422(crate::models::InlineResponse422),
    Status429(crate::models::InlineResponse429),
    Status500(crate::models::InlineResponse500),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_zilliqa_transaction_details_by_transaction_id`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetZilliqaTransactionDetailsByTransactionIdError {
    Status400(crate::models::InlineResponse4009),
    Status401(crate::models::InlineResponse4019),
    Status402(crate::models::InlineResponse402),
    Status403(crate::models::InlineResponse4039),
    Status404(crate::models::InlineResponse404),
    Status409(crate::models::InlineResponse409),
    Status415(crate::models::InlineResponse415),
    Status422(crate::models::InlineResponse422),
    Status429(crate::models::InlineResponse429),
    Status500(crate::models::InlineResponse500),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `list_zilliqa_transactions_by_address`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListZilliqaTransactionsByAddressError {
    Status400(crate::models::InlineResponse40014),
    Status401(crate::models::InlineResponse40114),
    Status402(crate::models::InlineResponse402),
    Status403(crate::models::InlineResponse40314),
    Status409(crate::models::InlineResponse409),
    Status415(crate::models::InlineResponse415),
    Status422(crate::models::InlineResponse422),
    Status429(crate::models::InlineResponse429),
    Status500(crate::models::InlineResponse500),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `list_zilliqa_transactions_by_block_hash`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListZilliqaTransactionsByBlockHashError {
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

/// struct for typed errors of method `list_zilliqa_transactions_by_block_height`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListZilliqaTransactionsByBlockHeightError {
    Status400(crate::models::InlineResponse40022),
    Status401(crate::models::InlineResponse40122),
    Status402(crate::models::InlineResponse402),
    Status403(crate::models::InlineResponse40322),
    Status409(crate::models::InlineResponse409),
    Status415(crate::models::InlineResponse415),
    Status422(crate::models::InlineResponse422),
    Status429(crate::models::InlineResponse429),
    Status500(crate::models::InlineResponse500),
    UnknownValue(serde_json::Value),
}


/// Through this endpoint users can obtain information on the latest block that has been mined on the Zilliqa blockchain. Data could include the current and previous block hashes, transaction count, and more.
pub async fn get_latest_mined_zilliqa_block(configuration: &configuration::Configuration, network: &str, context: Option<&str>) -> Result<crate::models::GetLatestMinedZilliqaBlockR, Error<GetLatestMinedZilliqaBlockError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/blockchain-data/zilliqa-specific/{network}/blocks/last", configuration.base_path, network=crate::apis::urlencode(network));
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
        let local_var_entity: Option<GetLatestMinedZilliqaBlockError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Through this endpoint customers can obtain information address details from the Zilliqa blockchain.
pub async fn get_zilliqa_address_details(configuration: &configuration::Configuration, network: &str, address: &str, context: Option<&str>) -> Result<crate::models::GetZilliqaAddressDetailsR, Error<GetZilliqaAddressDetailsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/blockchain-data/zilliqa-specific/{network}/addresses/{address}", configuration.base_path, network=crate::apis::urlencode(network), address=crate::apis::urlencode(address));
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
        let local_var_entity: Option<GetZilliqaAddressDetailsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Through this endpoint customers can obtain block details from the Zilliqa blockchain by providing the block Hash parameter.
pub async fn get_zilliqa_block_details_by_block_hash(configuration: &configuration::Configuration, network: &str, block_hash: &str, context: Option<&str>) -> Result<crate::models::GetZilliqaBlockDetailsByBlockHashR, Error<GetZilliqaBlockDetailsByBlockHashError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/blockchain-data/zilliqa-specific/{network}/blocks/hash/{blockHash}", configuration.base_path, network=crate::apis::urlencode(network), blockHash=crate::apis::urlencode(block_hash));
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
        let local_var_entity: Option<GetZilliqaBlockDetailsByBlockHashError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Through this endpoint customers can obtain block details from the Zilliqa blockchain by providing the block Height parameter.
pub async fn get_zilliqa_block_details_by_block_height(configuration: &configuration::Configuration, network: &str, block_height: i32, context: Option<&str>) -> Result<crate::models::GetZilliqaBlockDetailsByBlockHeightR, Error<GetZilliqaBlockDetailsByBlockHeightError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/blockchain-data/zilliqa-specific/{network}/blocks/height/{blockHeight}", configuration.base_path, network=crate::apis::urlencode(network), blockHeight=block_height);
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
        let local_var_entity: Option<GetZilliqaBlockDetailsByBlockHeightError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Through this endpoint customers can obtain transaction details on the Zilliqa blockchain by providing a Transaction ID parameter.
pub async fn get_zilliqa_transaction_details_by_transaction_id(configuration: &configuration::Configuration, network: &str, transaction_hash: &str, context: Option<&str>) -> Result<crate::models::GetZilliqaTransactionDetailsByTransactionIdr, Error<GetZilliqaTransactionDetailsByTransactionIdError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/blockchain-data/zilliqa-specific/{network}/transactions/{transactionHash}", configuration.base_path, network=crate::apis::urlencode(network), transactionHash=crate::apis::urlencode(transaction_hash));
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
        let local_var_entity: Option<GetZilliqaTransactionDetailsByTransactionIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Through this endpoint customers can list transactions on the Zilliqa blockchain by the address parameter.
pub async fn list_zilliqa_transactions_by_address(configuration: &configuration::Configuration, network: &str, address: &str, context: Option<&str>, limit: Option<i32>, offset: Option<i32>) -> Result<crate::models::ListZilliqaTransactionsByAddressR, Error<ListZilliqaTransactionsByAddressError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/blockchain-data/zilliqa-specific/{network}/addresses/{address}/transactions", configuration.base_path, network=crate::apis::urlencode(network), address=crate::apis::urlencode(address));
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
        let local_var_entity: Option<ListZilliqaTransactionsByAddressError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Through this endpoint customers can list transactions on the Zilliqa blockchain by the block hash parameter.
pub async fn list_zilliqa_transactions_by_block_hash(configuration: &configuration::Configuration, network: &str, block_hash: &str, context: Option<&str>, limit: Option<i32>, offset: Option<i32>) -> Result<crate::models::ListZilliqaTransactionsByBlockHashR, Error<ListZilliqaTransactionsByBlockHashError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/blockchain-data/zilliqa-specific/{network}/blocks/hash/{blockHash}/transactions", configuration.base_path, network=crate::apis::urlencode(network), blockHash=crate::apis::urlencode(block_hash));
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
        let local_var_entity: Option<ListZilliqaTransactionsByBlockHashError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Through this endpoint customers can list transactions on the Zilliqa blockchain by the block height parameter.
pub async fn list_zilliqa_transactions_by_block_height(configuration: &configuration::Configuration, network: &str, block_height: i32, context: Option<&str>, limit: Option<i32>, offset: Option<i32>) -> Result<crate::models::ListZilliqaTransactionsByBlockHeightR, Error<ListZilliqaTransactionsByBlockHeightError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/blockchain-data/zilliqa-specific/{network}/blocks/height/{blockHeight}/transactions", configuration.base_path, network=crate::apis::urlencode(network), blockHeight=block_height);
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
        let local_var_entity: Option<ListZilliqaTransactionsByBlockHeightError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

