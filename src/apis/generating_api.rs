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


/// struct for typed errors of method [`generate_deposit_address`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GenerateDepositAddressError {
    Status400(crate::models::InvalidPagination),
    Status401(crate::models::InvalidApiKey),
    Status402(crate::models::InsufficientCredits),
    Status403(crate::models::WalletAsAServiceDepositAddressesLimitReached),
    Status404(crate::models::ResourceNotFound),
    Status409(crate::models::InvalidData),
    Status415(crate::models::UnsupportedMediaType),
    Status422(crate::models::InvalidRequestBodyStructure),
    Status429(crate::models::RequestLimitReached),
    Status500(crate::models::UnexpectedServerError),
    UnknownValue(serde_json::Value),
}


/// Through this endpoint customers can generate a new Receiving/Deposit Addresses into their Wallet.
pub async fn generate_deposit_address(configuration: &configuration::Configuration, blockchain: &str, network: &str, wallet_id: &str, context: Option<&str>, generate_deposit_address_rb: Option<crate::models::GenerateDepositAddressRb>) -> Result<crate::models::GenerateDepositAddressR, Error<GenerateDepositAddressError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/wallet-as-a-service/wallets/{walletId}/{blockchain}/{network}/addresses", local_var_configuration.base_path, blockchain=crate::apis::urlencode(blockchain), network=crate::apis::urlencode(network), walletId=crate::apis::urlencode(wallet_id));
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
    local_var_req_builder = local_var_req_builder.json(&generate_deposit_address_rb);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GenerateDepositAddressError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

