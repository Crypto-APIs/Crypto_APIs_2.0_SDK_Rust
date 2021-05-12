# \UTXOBasedApi

All URIs are relative to *https://rest.cryptoapis.io/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_hd_wallet__x_pub_y_pub_z_pub_details**](UTXOBasedApi.md#get_hd_wallet__x_pub_y_pub_z_pub_details) | **get** /blockchain-data/{blockchain}/{network}/hd/{extendedPublicKey}/details | Get HD Wallet (xPub, yPub, zPub) Details
[**list_hd_wallet__x_pub_y_pub_z_pub_transactions**](UTXOBasedApi.md#list_hd_wallet__x_pub_y_pub_z_pub_transactions) | **get** /blockchain-data/{blockchain}/{network}/hd/{extendedPublicKey}/transactions | List HD Wallet (xPub, yPub, zPub) Transactions
[**sync_hd_wallet__x_pub_y_pub_z_pub**](UTXOBasedApi.md#sync_hd_wallet__x_pub_y_pub_z_pub) | **post** /blockchain-data/{blockchain}/{network}/hd/sync | Sync HD Wallet (xPub, yPub, zPub)



## get_hd_wallet__x_pub_y_pub_z_pub_details

> crate::models::GetHdWalletxPubYPubZPubDetailsResponse get_hd_wallet__x_pub_y_pub_z_pub_details(blockchain, extended_public_key, network, context, derivation)
Get HD Wallet (xPub, yPub, zPub) Details

HD wallet details is useful endpoint to get the most important data about HD wallet without the need to do a lot of calculations, once the HD Wallet is synced using Sync endpoint we keep it up to date and we calculate these details in advance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc. | [required] |
**extended_public_key** | **String** | Defines the account extended publicly known key which is used to derive all child public keys. | [required] |
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\", \"rinkeby\" are test networks. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |
**derivation** | Option<**String**> | The way how the HD walled derives, for example when the type is ACCOUNT, it derives change and receive addresses while when the type is BIP32 it derives directly. |  |

### Return type

[**crate::models::GetHdWalletxPubYPubZPubDetailsResponse**](GetHDWalletxPubYPubZPubDetailsResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_hd_wallet__x_pub_y_pub_z_pub_transactions

> crate::models::ListHdWalletxPubYPubZPubTransactionsResponse list_hd_wallet__x_pub_y_pub_z_pub_transactions(blockchain, extended_public_key, network, context, derivation, limit, offset)
List HD Wallet (xPub, yPub, zPub) Transactions

This endpoint will list HD Wallet transactions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blockchain** | **String** | Represents the specific blockchain. | [required] |
**extended_public_key** | **String** | Defines the master public key (xPub) of the account. | [required] |
**network** | **String** | Represents the specific network. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |
**derivation** | Option<**String**> | The way how the HD walled derives, for example when the type is ACCOUNT, it derives change and receive addresses while when the type is BIP32 it derives directly. |  |
**limit** | Option<**i32**> | Defines how many items should be returned in the response per page basis. |  |[default to 50]
**offset** | Option<**i32**> | The starting index of the response items, i.e. where the response should start listing the returned items. |  |[default to 0]

### Return type

[**crate::models::ListHdWalletxPubYPubZPubTransactionsResponse**](ListHDWalletxPubYPubZPubTransactionsResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sync_hd_wallet__x_pub_y_pub_z_pub

> crate::models::SyncHdWalletxPubYPubZPubResponse sync_hd_wallet__x_pub_y_pub_z_pub(blockchain, network, context, sync_hd_walletx_pub_y_pub_z_pub_request_body)
Sync HD Wallet (xPub, yPub, zPub)

HD wallets usually have a lot of addresses and transactions, getting the data on demand is a heavy operation. That's why we have created this feature, to be able to get HD wallet details or transactions this HD wallet must be synced first. In addition to the initial sync we keep updating the synced HD wallets all the time.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc. | [required] |
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\", \"rinkeby\" are test networks. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |
**sync_hd_walletx_pub_y_pub_z_pub_request_body** | Option<[**SyncHdWalletxPubYPubZPubRequestBody**](SyncHdWalletxPubYPubZPubRequestBody.md)> |  |  |

### Return type

[**crate::models::SyncHdWalletxPubYPubZPubResponse**](SyncHDWalletxPubYPubZPubResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

