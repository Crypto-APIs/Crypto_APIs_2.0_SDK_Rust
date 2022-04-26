# \XRPRippleApi

All URIs are relative to *https://rest.cryptoapis.io/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_latest_mined_xrp__ripple_block**](XRPRippleApi.md#get_latest_mined_xrp__ripple_block) | **GET** /blockchain-data/xrp-specific/{network}/blocks/last | Get Latest Mined XRP (Ripple) Block
[**get_xrp__ripple_address_details**](XRPRippleApi.md#get_xrp__ripple_address_details) | **GET** /blockchain-data/xrp-specific/{network}/addresses/{address} | Get XRP (Ripple) Address Details
[**get_xrp__ripple_block_details_by_block_hash**](XRPRippleApi.md#get_xrp__ripple_block_details_by_block_hash) | **GET** /blockchain-data/xrp-specific/{network}/blocks/hash/{blockHash} | Get XRP (Ripple) Block Details By Block Hash
[**get_xrp__ripple_block_details_by_block_height**](XRPRippleApi.md#get_xrp__ripple_block_details_by_block_height) | **GET** /blockchain-data/xrp-specific/{network}/blocks/height/{blockHeight} | Get XRP (Ripple) Block Details By Block Height
[**get_xrp__ripple_transaction_details_by_transaction_id**](XRPRippleApi.md#get_xrp__ripple_transaction_details_by_transaction_id) | **GET** /blockchain-data/xrp-specific/{network}/transactions/{transactionHash} | Get XRP (Ripple) Transaction Details By Transaction ID
[**list_xrp__ripple_transactions_by_address**](XRPRippleApi.md#list_xrp__ripple_transactions_by_address) | **GET** /blockchain-data/xrp-specific/{network}/addresses/{address}/transactions | List XRP (Ripple) Transactions by Address
[**list_xrp__ripple_transactions_by_address_and_time_range**](XRPRippleApi.md#list_xrp__ripple_transactions_by_address_and_time_range) | **GET** /blockchain-data/xrp-specific/{network}/addresses/{address}/transactions-by-time-range | List XRP (Ripple) Transactions By Address And Time Range
[**list_xrp__ripple_transactions_by_block_hash**](XRPRippleApi.md#list_xrp__ripple_transactions_by_block_hash) | **GET** /blockchain-data/xrp-specific/{network}/blocks/hash/{blockHash}/transactions | List XRP (Ripple) Transactions By Block Hash
[**list_xrp__ripple_transactions_by_block_height**](XRPRippleApi.md#list_xrp__ripple_transactions_by_block_height) | **GET** /blockchain-data/xrp-specific/{network}/blocks/height/{blockHeight}/transactions | List XRP (Ripple) Transactions By Block Height



## get_latest_mined_xrp__ripple_block

> crate::models::GetLatestMinedXrpRippleBlockR get_latest_mined_xrp__ripple_block(network, context)
Get Latest Mined XRP (Ripple) Block

Through this endpoint customers can fetch the last mined XRP block in the blockchain, along with its details. These could include the hash of the specific, the previous and the next block, its transactions count, its height, etc.     Since XRP is a different blockchain than Bitcoin and Ethereum, it isn't unified.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |

### Return type

[**crate::models::GetLatestMinedXrpRippleBlockR**](GetLatestMinedXRPRippleBlockR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_xrp__ripple_address_details

> crate::models::GetXrpRippleAddressDetailsR get_xrp__ripple_address_details(network, address, context)
Get XRP (Ripple) Address Details

Through this endpoint the customer can receive basic information about a given XRP address based on confirmed/synced blocks only. In the case where there are any incoming or outgoing **unconfirmed** transactions for the specific address, they **will not** be counted or calculated here.    Since XRP is a different blockchain than Bitcoin and Ethereum, it isn't unified.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\",  are test networks. | [required] |
**address** | **String** | Represents the public address, which is a compressed and shortened form of a public key. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |

### Return type

[**crate::models::GetXrpRippleAddressDetailsR**](GetXRPRippleAddressDetailsR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_xrp__ripple_block_details_by_block_hash

> crate::models::GetXrpRippleBlockDetailsByBlockHashR get_xrp__ripple_block_details_by_block_hash(network, block_hash, context)
Get XRP (Ripple) Block Details By Block Hash

Through this endpoint customers can obtain basic information about a given XRP block (a block on the XRP blockchain), specifically by using the `hash` parameter. These block details could include the hash of the specific, the previous and the next block, the number of included transactions, etc.     Since XRP is a different blockchain than Bitcoin and Ethereum, it isn't unified.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**block_hash** | **String** | Represents the hash of the block, which is its unique identifier. It represents a cryptographic digital fingerprint made by hashing the block header twice through the SHA256 algorithm. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |

### Return type

[**crate::models::GetXrpRippleBlockDetailsByBlockHashR**](GetXRPRippleBlockDetailsByBlockHashR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_xrp__ripple_block_details_by_block_height

> crate::models::GetXrpRippleBlockDetailsByBlockHeightR get_xrp__ripple_block_details_by_block_height(network, block_height, context)
Get XRP (Ripple) Block Details By Block Height

Through this endpoint customers can obtain basic information about a given XRP block (a block on the XRP blockchain), specifically by using the `height` parameter. These block details could include the hash of the specific, the previous and the next block, its transactions count, etc.    Since XRP is a different blockchain than Bitcoin and Ethereum, it isn't unified.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\",  are test networks. | [required] |
**block_height** | **String** | Represents the number of blocks in the blockchain preceding this specific block. Block numbers have no gaps. A blockchain usually starts with block 0 called the \"Genesis block\". | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |

### Return type

[**crate::models::GetXrpRippleBlockDetailsByBlockHeightR**](GetXRPRippleBlockDetailsByBlockHeightR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_xrp__ripple_transaction_details_by_transaction_id

> crate::models::GetXrpRippleTransactionDetailsByTransactionIdr get_xrp__ripple_transaction_details_by_transaction_id(network, transaction_hash, context)
Get XRP (Ripple) Transaction Details By Transaction ID

Through this endpoint customers can obtain details about a XRP transaction by the transaction's unique identifier.     Since XRP is a different blockchain than Bitcoin and Ethereum, it isn't unified.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**transaction_hash** | **String** | Represents the same as `transactionId` for account-based protocols like Ethereum, while it could be different in UTXO-based protocols like Bitcoin. E.g., in UTXO-based protocols `hash` is different from `transactionId` for SegWit transactions. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |

### Return type

[**crate::models::GetXrpRippleTransactionDetailsByTransactionIdr**](GetXRPRippleTransactionDetailsByTransactionIDR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_xrp__ripple_transactions_by_address

> crate::models::ListXrpRippleTransactionsByAddressR list_xrp__ripple_transactions_by_address(network, address, context, limit, offset, transaction_type)
List XRP (Ripple) Transactions by Address

This endpoint will list XRP transactions by a attribute `address`. The transactions listed will detail additional information such as hash, height, time of creation in Unix timestamp, etc.    Since XRP is a different blockchain than Bitcoin and Ethereum, it isn't unified.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**address** | **String** | Represents the public address, which is a compressed and shortened form of a public key. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |
**limit** | Option<**i64**> | Defines how many items should be returned in the response per page basis. |  |[default to 50]
**offset** | Option<**i64**> | The starting index of the response items, i.e. where the response should start listing the returned items. |  |[default to 0]
**transaction_type** | Option<**String**> |  |  |

### Return type

[**crate::models::ListXrpRippleTransactionsByAddressR**](ListXRPRippleTransactionsByAddressR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_xrp__ripple_transactions_by_address_and_time_range

> crate::models::ListXrpRippleTransactionsByAddressAndTimeRangeR list_xrp__ripple_transactions_by_address_and_time_range(network, address, from_timestamp, to_timestamp, context, limit, offset, transaction_type)
List XRP (Ripple) Transactions By Address And Time Range

Тhis endpoint lists XRP transactions by the attribute `address` and the query parameters `fromTimestamp` and `toTimestamp`  which gives customers the opportunity to filter the results by a specified time period.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**address** | **String** | Represents the public address, which is a compressed and shortened form of a public key. | [required] |
**from_timestamp** | **i32** | Defines the specific time/date from which the results will start being listed. | [required] |
**to_timestamp** | **i32** | Defines the specific time/date to which the results will be listed. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |
**limit** | Option<**i64**> | Defines how many items should be returned in the response per page basis. |  |[default to 50]
**offset** | Option<**i64**> | The starting index of the response items, i.e. where the response should start listing the returned items. |  |[default to 0]
**transaction_type** | Option<**String**> | Defines the transaction type. |  |

### Return type

[**crate::models::ListXrpRippleTransactionsByAddressAndTimeRangeR**](ListXRPRippleTransactionsByAddressAndTimeRangeR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_xrp__ripple_transactions_by_block_hash

> crate::models::ListXrpRippleTransactionsByBlockHashR list_xrp__ripple_transactions_by_block_hash(network, block_hash, context, limit, offset)
List XRP (Ripple) Transactions By Block Hash

This endpoint will list transactions by an attribute `blockHash`. The transactions listed will detail additional information such as hash, addresses, time of creation in Unix timestamp, etc.    Since XRP is a different blockchain than Bitcoin and Ethereum, it isn't unified.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**block_hash** | **String** | Represents the hash of the block, which is its unique identifier. It represents a cryptographic digital fingerprint made by hashing the block header twice through the SHA256 algorithm. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |
**limit** | Option<**i64**> | Defines how many items should be returned in the response per page basis. |  |[default to 50]
**offset** | Option<**i64**> | The starting index of the response items, i.e. where the response should start listing the returned items. |  |[default to 0]

### Return type

[**crate::models::ListXrpRippleTransactionsByBlockHashR**](ListXRPRippleTransactionsByBlockHashR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_xrp__ripple_transactions_by_block_height

> crate::models::ListXrpRippleTransactionsByBlockHeightR list_xrp__ripple_transactions_by_block_height(network, block_height, context, limit, offset)
List XRP (Ripple) Transactions By Block Height

This endpoint will list transactions by an attribute `blockHeight`. The transactions listed will detail additional information such as hash, addresses, time of creation in Unix timestamp, etc.    Since XRP is a different blockchain than Bitcoin and Ethereum, it isn't unified.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**block_height** | **i64** |  | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |
**limit** | Option<**i64**> | Defines how many items should be returned in the response per page basis. |  |[default to 50]
**offset** | Option<**i64**> | The starting index of the response items, i.e. where the response should start listing the returned items. |  |[default to 0]

### Return type

[**crate::models::ListXrpRippleTransactionsByBlockHeightR**](ListXRPRippleTransactionsByBlockHeightR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

