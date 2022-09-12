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


/// struct for typed errors of method `derive_and_sync_new_change_addresses`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeriveAndSyncNewChangeAddressesError {
    Status400(crate::models::InlineResponse40050),
    Status401(crate::models::InlineResponse40150),
    Status402(crate::models::InlineResponse402),
    Status403(crate::models::InlineResponse40350),
    Status409(crate::models::InlineResponse409),
    Status415(crate::models::InlineResponse415),
    Status422(crate::models::InlineResponse422),
    Status429(crate::models::InlineResponse429),
    Status500(crate::models::InlineResponse500),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `derive_and_sync_new_receiving_addresses`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeriveAndSyncNewReceivingAddressesError {
    Status400(crate::models::InlineResponse40049),
    Status401(crate::models::InlineResponse40149),
    Status402(crate::models::InlineResponse402),
    Status403(crate::models::InlineResponse40349),
    Status409(crate::models::InlineResponse409),
    Status415(crate::models::InlineResponse415),
    Status422(crate::models::InlineResponse422),
    Status429(crate::models::InlineResponse429),
    Status500(crate::models::InlineResponse500),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_hd_wallet__x_pub_y_pub_z_pub_assets`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetHdWalletXPubYPubZPubAssetsError {
    Status400(crate::models::InlineResponse40069),
    Status401(crate::models::InlineResponse40169),
    Status402(crate::models::InlineResponse402),
    Status403(crate::models::InlineResponse40369),
    Status409(crate::models::InlineResponse409),
    Status415(crate::models::InlineResponse415),
    Status422(crate::models::InlineResponse4225),
    Status429(crate::models::InlineResponse429),
    Status500(crate::models::InlineResponse500),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `get_hd_wallet__x_pub_y_pub_z_pub_details`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetHdWalletXPubYPubZPubDetailsError {
    Status400(crate::models::InlineResponse40070),
    Status401(crate::models::InlineResponse40170),
    Status402(crate::models::InlineResponse402),
    Status403(crate::models::InlineResponse40370),
    Status409(crate::models::InlineResponse409),
    Status415(crate::models::InlineResponse415),
    Status422(crate::models::InlineResponse4226),
    Status429(crate::models::InlineResponse429),
    Status500(crate::models::InlineResponse500),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `list_hd_wallet__x_pub_y_pub_z_pub_transactions`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListHdWalletXPubYPubZPubTransactionsError {
    Status400(crate::models::InlineResponse40055),
    Status401(crate::models::InlineResponse40155),
    Status402(crate::models::InlineResponse402),
    Status403(crate::models::InlineResponse40355),
    Status409(crate::models::InlineResponse409),
    Status415(crate::models::InlineResponse415),
    Status422(crate::models::InlineResponse4223),
    Status429(crate::models::InlineResponse429),
    Status500(crate::models::InlineResponse500),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `list_hd_wallet__x_pub_y_pub_z_pub_utxos`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListHdWalletXPubYPubZPubUtxosError {
    Status400(crate::models::InlineResponse40061),
    Status401(crate::models::InlineResponse40161),
    Status402(crate::models::InlineResponse402),
    Status403(crate::models::InlineResponse40361),
    Status409(crate::models::InlineResponse409),
    Status415(crate::models::InlineResponse415),
    Status422(crate::models::InlineResponse4224),
    Status429(crate::models::InlineResponse429),
    Status500(crate::models::InlineResponse500),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `list_synced_addresses`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListSyncedAddressesError {
    Status400(crate::models::InlineResponse40052),
    Status401(crate::models::InlineResponse40152),
    Status402(crate::models::InlineResponse402),
    Status403(crate::models::InlineResponse40352),
    Status409(crate::models::InlineResponse409),
    Status415(crate::models::InlineResponse415),
    Status422(crate::models::InlineResponse422),
    Status429(crate::models::InlineResponse429),
    Status500(crate::models::InlineResponse500),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `prepare_a_utxo_based_transaction_from_hd_wallet__x_pub_y_pub_z_pub`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PrepareAUtxoBasedTransactionFromHdWalletXPubYPubZPubError {
    Status400(crate::models::InlineResponse40066),
    Status401(crate::models::InlineResponse40166),
    Status402(crate::models::InlineResponse402),
    Status403(crate::models::InlineResponse40366),
    Status409(crate::models::InlineResponse409),
    Status415(crate::models::InlineResponse415),
    Status422(crate::models::InlineResponse422),
    Status429(crate::models::InlineResponse429),
    Status500(crate::models::InlineResponse500),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `prepare_an_account_based_transaction_from_hd_wallet__x_pub_y_pub_z_pub`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PrepareAnAccountBasedTransactionFromHdWalletXPubYPubZPubError {
    Status400(crate::models::InlineResponse40068),
    Status401(crate::models::InlineResponse40168),
    Status402(crate::models::InlineResponse402),
    Status403(crate::models::InlineResponse40368),
    Status409(crate::models::InlineResponse409),
    Status415(crate::models::InlineResponse415),
    Status422(crate::models::InlineResponse422),
    Status429(crate::models::InlineResponse429),
    Status500(crate::models::InlineResponse500),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `sync_hd_wallet__x_pub_y_pub_z_pub`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SyncHdWalletXPubYPubZPubError {
    Status400(crate::models::InlineResponse40041),
    Status401(crate::models::InlineResponse40141),
    Status402(crate::models::InlineResponse402),
    Status403(crate::models::InlineResponse40341),
    Status409(crate::models::InlineResponse4096),
    Status415(crate::models::InlineResponse415),
    Status422(crate::models::InlineResponse4221),
    Status429(crate::models::InlineResponse429),
    Status500(crate::models::InlineResponse500),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `sync_new_hd_wallet__x_pub_y_pub_z_pub`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SyncNewHdWalletXPubYPubZPubError {
    Status400(crate::models::InlineResponse40046),
    Status401(crate::models::InlineResponse40146),
    Status402(crate::models::InlineResponse402),
    Status403(crate::models::InlineResponse40346),
    Status409(crate::models::InlineResponse4098),
    Status415(crate::models::InlineResponse415),
    Status422(crate::models::InlineResponse4222),
    Status429(crate::models::InlineResponse429),
    Status500(crate::models::InlineResponse500),
    UnknownValue(serde_json::Value),
}


/// Through this endpoint users can derive 100 change addresses, starting from the last index we have data for, which are then added to the xPub, subscribed for syncing, and start recording data. If no data is available, it will start from index 0.
pub async fn derive_and_sync_new_change_addresses(configuration: &configuration::Configuration, blockchain: &str, network: &str, context: Option<&str>, derive_and_sync_new_change_addresses_rb: Option<crate::models::DeriveAndSyncNewChangeAddressesRb>) -> Result<crate::models::DeriveAndSyncNewChangeAddressesR, Error<DeriveAndSyncNewChangeAddressesError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/blockchain-data/{blockchain}/{network}/hd/derive-sync-change", configuration.base_path, blockchain=crate::apis::urlencode(blockchain), network=crate::apis::urlencode(network));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

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
    local_var_req_builder = local_var_req_builder.json(&derive_and_sync_new_change_addresses_rb);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DeriveAndSyncNewChangeAddressesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Through this endpoint users can derive 100 receiving addresses, starting from the last index we have data for, which are then added to the xPub, subscribed for syncing, and start recording data. If no data is available, it will start from index 0.
pub async fn derive_and_sync_new_receiving_addresses(configuration: &configuration::Configuration, blockchain: &str, network: &str, context: Option<&str>, derive_and_sync_new_receiving_addresses_rb: Option<crate::models::DeriveAndSyncNewReceivingAddressesRb>) -> Result<crate::models::DeriveAndSyncNewReceivingAddressesR, Error<DeriveAndSyncNewReceivingAddressesError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/blockchain-data/{blockchain}/{network}/hd/derive-and-sync", configuration.base_path, blockchain=crate::apis::urlencode(blockchain), network=crate::apis::urlencode(network));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

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
    local_var_req_builder = local_var_req_builder.json(&derive_and_sync_new_receiving_addresses_rb);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DeriveAndSyncNewReceivingAddressesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// This endpoint will return details on assets we support for a specified from the customer extended public key (xPub). These could be cryptocurrencies, fungible or non-fungible (NFT) tokens. Each asset has a unique identifier - assetId, and a unique symbol in the form of a string, e.g. \"USDT\".
pub async fn get_hd_wallet__x_pub_y_pub_z_pub_assets(configuration: &configuration::Configuration, blockchain: &str, extended_public_key: &str, network: &str, context: Option<&str>, derivation: Option<&str>) -> Result<crate::models::GetHdWalletXPubYPubZPubAssetsR, Error<GetHdWalletXPubYPubZPubAssetsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/blockchain-data/{blockchain}/{network}/hd/{extendedPublicKey}/assets", configuration.base_path, blockchain=crate::apis::urlencode(blockchain), extendedPublicKey=crate::apis::urlencode(extended_public_key), network=crate::apis::urlencode(network));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = context {
        local_var_req_builder = local_var_req_builder.query(&[("context", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = derivation {
        local_var_req_builder = local_var_req_builder.query(&[("derivation", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetHdWalletXPubYPubZPubAssetsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// HD wallet details is useful endpoint to get the most important data about HD wallet without the need to do a lot of calculations, once the HD Wallet is synced using Sync endpoint we keep it up to date and we calculate these details in advance.
pub async fn get_hd_wallet__x_pub_y_pub_z_pub_details(configuration: &configuration::Configuration, blockchain: &str, extended_public_key: &str, network: &str, context: Option<&str>, derivation: Option<&str>) -> Result<crate::models::GetHdWalletXPubYPubZPubDetailsR, Error<GetHdWalletXPubYPubZPubDetailsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/blockchain-data/{blockchain}/{network}/hd/{extendedPublicKey}/details", configuration.base_path, blockchain=crate::apis::urlencode(blockchain), extendedPublicKey=crate::apis::urlencode(extended_public_key), network=crate::apis::urlencode(network));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = context {
        local_var_req_builder = local_var_req_builder.query(&[("context", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = derivation {
        local_var_req_builder = local_var_req_builder.query(&[("derivation", &local_var_str.to_string())]);
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
        let local_var_entity: Option<GetHdWalletXPubYPubZPubDetailsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// This endpoint will list HD Wallet transactions.
pub async fn list_hd_wallet__x_pub_y_pub_z_pub_transactions(configuration: &configuration::Configuration, blockchain: &str, extended_public_key: &str, network: &str, context: Option<&str>, derivation: Option<&str>, limit: Option<i64>, offset: Option<i64>) -> Result<crate::models::ListHdWalletXPubYPubZPubTransactionsR, Error<ListHdWalletXPubYPubZPubTransactionsError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/blockchain-data/{blockchain}/{network}/hd/{extendedPublicKey}/transactions", configuration.base_path, blockchain=crate::apis::urlencode(blockchain), extendedPublicKey=crate::apis::urlencode(extended_public_key), network=crate::apis::urlencode(network));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = context {
        local_var_req_builder = local_var_req_builder.query(&[("context", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = derivation {
        local_var_req_builder = local_var_req_builder.query(&[("derivation", &local_var_str.to_string())]);
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
        let local_var_entity: Option<ListHdWalletXPubYPubZPubTransactionsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Through this endpoint you can list HD wallet's UTXOs (Unspent Transaction Outputs) by providing extended public key of an already synced HD wallet.
pub async fn list_hd_wallet__x_pub_y_pub_z_pub_utxos(configuration: &configuration::Configuration, blockchain: &str, extended_public_key: &str, network: &str, context: Option<&str>, derivation: Option<&str>, limit: Option<i64>, offset: Option<i64>) -> Result<crate::models::ListHdWalletXPubYPubZPubUtxosR, Error<ListHdWalletXPubYPubZPubUtxosError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/blockchain-data/{blockchain}/{network}/hd/{extendedPublicKey}/utxos", configuration.base_path, blockchain=crate::apis::urlencode(blockchain), extendedPublicKey=crate::apis::urlencode(extended_public_key), network=crate::apis::urlencode(network));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = context {
        local_var_req_builder = local_var_req_builder.query(&[("context", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = derivation {
        local_var_req_builder = local_var_req_builder.query(&[("derivation", &local_var_str.to_string())]);
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
        let local_var_entity: Option<ListHdWalletXPubYPubZPubUtxosError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Through this endpoint users can list all addresses that Crypto APIs has synced for a specific xPub. This includes previous and current/new xPubs, what addresses we’ve synced for them, etc.
pub async fn list_synced_addresses(configuration: &configuration::Configuration, blockchain: &str, extended_public_key: &str, network: &str, context: Option<&str>, address_format: Option<&str>, is_change_address: Option<bool>, limit: Option<i64>, offset: Option<i64>) -> Result<crate::models::ListSyncedAddressesR, Error<ListSyncedAddressesError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/blockchain-data/{blockchain}/{network}/hd/{extendedPublicKey}/synced-addresses", configuration.base_path, blockchain=crate::apis::urlencode(blockchain), extendedPublicKey=crate::apis::urlencode(extended_public_key), network=crate::apis::urlencode(network));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = context {
        local_var_req_builder = local_var_req_builder.query(&[("context", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = address_format {
        local_var_req_builder = local_var_req_builder.query(&[("addressFormat", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = is_change_address {
        local_var_req_builder = local_var_req_builder.query(&[("isChangeAddress", &local_var_str.to_string())]);
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
        let local_var_entity: Option<ListSyncedAddressesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Through the “Prepare a UTXO-based transaction from xPub” endpoint users can prepare a transaction for signing from all synced with Crypto APIs addresses for the specific xPub. This is based on the `selectionStrategy` and the addresses’ balances. In the case a user has an address not synced with Crypto APIs, it will not be included. This endpoint applies to all supported UTXO-based blockchain protocols, e.g. Bitcoin, Litecoin, etc.
pub async fn prepare_a_utxo_based_transaction_from_hd_wallet__x_pub_y_pub_z_pub(configuration: &configuration::Configuration, blockchain: &str, network: &str, context: Option<&str>, prepare_autxo_based_transaction_from_hd_wallet_x_pub_y_pub_z_pub_rb: Option<crate::models::PrepareAutxoBasedTransactionFromHdWalletXPubYPubZPubRb>) -> Result<crate::models::PrepareAutxoBasedTransactionFromHdWalletXPubYPubZPubR, Error<PrepareAUtxoBasedTransactionFromHdWalletXPubYPubZPubError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/blockchain-data/{blockchain}/{network}/transactions/prepare-utxo-transaction", configuration.base_path, blockchain=crate::apis::urlencode(blockchain), network=crate::apis::urlencode(network));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

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
    local_var_req_builder = local_var_req_builder.json(&prepare_autxo_based_transaction_from_hd_wallet_x_pub_y_pub_z_pub_rb);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PrepareAUtxoBasedTransactionFromHdWalletXPubYPubZPubError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Through the “Prepare an account-based transaction from xPub” endpoint users can prepare a transaction for signing from a synced with Crypto APIs address from the specific xPub. This endpoint applies to all supported account-based blockchain protocols, e.g. Ethereum, BSC, etc
pub async fn prepare_an_account_based_transaction_from_hd_wallet__x_pub_y_pub_z_pub(configuration: &configuration::Configuration, blockchain: &str, network: &str, context: Option<&str>, prepare_an_account_based_transaction_from_hd_wallet_x_pub_y_pub_z_pub_rb: Option<crate::models::PrepareAnAccountBasedTransactionFromHdWalletXPubYPubZPubRb>) -> Result<crate::models::PrepareAnAccountBasedTransactionFromHdWalletXPubYPubZPubR, Error<PrepareAnAccountBasedTransactionFromHdWalletXPubYPubZPubError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/blockchain-data/{blockchain}/{network}/transactions/prepare-account-based-transaction", configuration.base_path, blockchain=crate::apis::urlencode(blockchain), network=crate::apis::urlencode(network));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

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
    local_var_req_builder = local_var_req_builder.json(&prepare_an_account_based_transaction_from_hd_wallet_x_pub_y_pub_z_pub_rb);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PrepareAnAccountBasedTransactionFromHdWalletXPubYPubZPubError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// HD wallets usually have a lot of addresses and transactions, getting the data on demand is a heavy operation. That's why we have created this feature, to be able to get HD wallet details or transactions this HD wallet must be synced first. In addition to the initial sync we keep updating the synced HD wallets all the time.
pub async fn sync_hd_wallet__x_pub_y_pub_z_pub(configuration: &configuration::Configuration, blockchain: &str, network: &str, context: Option<&str>, sync_hd_wallet_x_pub_y_pub_z_pub_rb: Option<crate::models::SyncHdWalletXPubYPubZPubRb>) -> Result<crate::models::SyncHdWalletXPubYPubZPubR, Error<SyncHdWalletXPubYPubZPubError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/blockchain-data/{blockchain}/{network}/hd/sync", configuration.base_path, blockchain=crate::apis::urlencode(blockchain), network=crate::apis::urlencode(network));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

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
    local_var_req_builder = local_var_req_builder.json(&sync_hd_wallet_x_pub_y_pub_z_pub_rb);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SyncHdWalletXPubYPubZPubError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Through this endpoint users can add a brand new xPub to the Crypto APIs system to be ready for deriving. Unlike our other similar endpoint “Sync HD Wallet (xPub, yPub, zPub)”, this endpoint does not create new addresses nor syncs old data.
pub async fn sync_new_hd_wallet__x_pub_y_pub_z_pub(configuration: &configuration::Configuration, blockchain: &str, network: &str, context: Option<&str>, sync_new_hd_wallet_x_pub_y_pub_z_pub_rb: Option<crate::models::SyncNewHdWalletXPubYPubZPubRb>) -> Result<crate::models::SyncNewHdWalletXPubYPubZPubR, Error<SyncNewHdWalletXPubYPubZPubError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/blockchain-data/{blockchain}/{network}/hd/sync-new", configuration.base_path, blockchain=crate::apis::urlencode(blockchain), network=crate::apis::urlencode(network));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

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
    local_var_req_builder = local_var_req_builder.json(&sync_new_hd_wallet_x_pub_y_pub_z_pub_rb);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SyncNewHdWalletXPubYPubZPubError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

