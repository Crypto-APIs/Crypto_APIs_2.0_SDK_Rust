# \UnifiedEndpointsApi

All URIs are relative to *https://rest.cryptoapis.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**estimate_transaction_smart_fee**](UnifiedEndpointsApi.md#estimate_transaction_smart_fee) | **GET** /blockchain-data/{blockchain}/{network}/estimate-transaction-smart-fee | Estimate Transaction Smart Fee
[**get_address_balance**](UnifiedEndpointsApi.md#get_address_balance) | **GET** /blockchain-data/{blockchain}/{network}/addresses/{address}/balance | Get Address Balance
[**get_address_details**](UnifiedEndpointsApi.md#get_address_details) | **GET** /blockchain-data/{blockchain}/{network}/addresses/{address} | Get Address Details
[**get_block_details_by_block_hash**](UnifiedEndpointsApi.md#get_block_details_by_block_hash) | **GET** /blockchain-data/{blockchain}/{network}/blocks/hash/{blockHash} | Get Block Details By Block Hash
[**get_block_details_by_block_height**](UnifiedEndpointsApi.md#get_block_details_by_block_height) | **GET** /blockchain-data/{blockchain}/{network}/blocks/height/{height} | Get Block Details By Block Height
[**get_fee_recommendations**](UnifiedEndpointsApi.md#get_fee_recommendations) | **GET** /blockchain-data/{blockchain}/{network}/mempool/fees | Get Fee Recommendations
[**get_last_mined_block**](UnifiedEndpointsApi.md#get_last_mined_block) | **GET** /blockchain-data/{blockchain}/{network}/blocks/last | Get Last Mined Block
[**get_next_available_nonce**](UnifiedEndpointsApi.md#get_next_available_nonce) | **GET** /blockchain-data/{blockchain}/{network}/addresses/{address}/next-available-nonce | Get Next Available Nonce
[**get_raw_transaction_data**](UnifiedEndpointsApi.md#get_raw_transaction_data) | **GET** /blockchain-data/{blockchain}/{network}/transactions/{transactionId}/raw-data | Get Raw Transaction Data
[**get_transaction_details_by_transaction_id**](UnifiedEndpointsApi.md#get_transaction_details_by_transaction_id) | **GET** /blockchain-data/{blockchain}/{network}/transactions/{transactionId} | Get Transaction Details By Transaction ID
[**list_confirmed_tokens_transfers_by_address_and_time_range**](UnifiedEndpointsApi.md#list_confirmed_tokens_transfers_by_address_and_time_range) | **GET** /blockchain-data/{blockchain}/{network}/addresses/{address}/tokens-transfers-by-time-range | List Confirmed Tokens Transfers By Address And Time Range
[**list_confirmed_transactions_by_address**](UnifiedEndpointsApi.md#list_confirmed_transactions_by_address) | **GET** /blockchain-data/{blockchain}/{network}/addresses/{address}/transactions | List Confirmed Transactions By Address
[**list_confirmed_transactions_by_address_and_time_range**](UnifiedEndpointsApi.md#list_confirmed_transactions_by_address_and_time_range) | **GET** /blockchain-data/{blockchain}/{network}/addresses/{address}/transactions-by-time-range | List Confirmed Transactions By Address And Time Range
[**list_internal_transactions_by_address_and_time_range**](UnifiedEndpointsApi.md#list_internal_transactions_by_address_and_time_range) | **GET** /blockchain-data/{blockchain}/{network}/addresses/{address}/internal-by-time-range | List Internal Transactions By Address And Time Range
[**list_latest_mined_blocks**](UnifiedEndpointsApi.md#list_latest_mined_blocks) | **GET** /blockchain-data/{blockchain}/{network}/blocks/last/{count} | List Latest Mined Blocks
[**list_transactions_by_block_hash**](UnifiedEndpointsApi.md#list_transactions_by_block_hash) | **GET** /blockchain-data/{blockchain}/{network}/blocks/hash/{blockHash}/transactions | List Transactions by Block Hash
[**list_transactions_by_block_height**](UnifiedEndpointsApi.md#list_transactions_by_block_height) | **GET** /blockchain-data/{blockchain}/{network}/blocks/height/{height}/transactions | List Transactions by Block Height
[**list_unconfirmed_transactions_by_address**](UnifiedEndpointsApi.md#list_unconfirmed_transactions_by_address) | **GET** /blockchain-data/{blockchain}/{network}/address-transactions-unconfirmed/{address} | List Unconfirmed Transactions by Address
[**list_unspent_transaction_outputs_by_address**](UnifiedEndpointsApi.md#list_unspent_transaction_outputs_by_address) | **GET** /blockchain-data/{blockchain}/{network}/addresses/{address}/unspent-outputs | List Unspent Transaction Outputs By Address



## estimate_transaction_smart_fee

> crate::models::EstimateTransactionSmartFeeR estimate_transaction_smart_fee(blockchain, network, context, confirmation_target, estimate_mode)
Estimate Transaction Smart Fee

Through this endpoint, customers can estimate the approximate fee per kilobyte needed for a transaction to begin confirmation within the `confirmationTarget` blocks when possible. After which it will return the number of blocks for which the estimate is valid.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blockchain** | **String** |  | [required] |
**network** | **String** |  | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |
**confirmation_target** | Option<**i32**> | Integer representation of confirmation target in blocks that estimation will be valid for |  |
**estimate_mode** | Option<**String**> | String representation of the address |  |

### Return type

[**crate::models::EstimateTransactionSmartFeeR**](EstimateTransactionSmartFeeR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_address_balance

> crate::models::GetAddressBalanceR get_address_balance(blockchain, network, address, context)
Get Address Balance

Through this endpoint the customer can receive the balance of a given address based on confirmed/synced blocks only. In the case where there are any incoming or outgoing unconfirmed transactions for the specific address, they will not be counted or calculated here. Applies only for coins.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc. | [required] |
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**address** | **String** | Represents the public address, which is a compressed and shortened form of a public key. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |

### Return type

[**crate::models::GetAddressBalanceR**](GetAddressBalanceR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_address_details

> crate::models::GetAddressDetailsR get_address_details(blockchain, network, address, context)
Get Address Details

Through this endpoint the customer can receive basic information about a given address based on confirmed/synced blocks only. In the case where there are any incoming or outgoing **unconfirmed** transactions for the specific address, they **will not** be counted or calculated here. Applies only for coins.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc. | [required] |
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**address** | **String** | Represents the public address, which is a compressed and shortened form of a public key. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |

### Return type

[**crate::models::GetAddressDetailsR**](GetAddressDetailsR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_block_details_by_block_hash

> crate::models::GetBlockDetailsByBlockHashR get_block_details_by_block_hash(blockchain, network, block_hash, context)
Get Block Details By Block Hash

Through this endpoint customers can obtain basic information about a given mined block, specifically by using the `hash` parameter. These block details could include the hash of the specific, the previous and the next block, its transactions count, its height, etc.     Blockchain specific data is information such as version, nonce, size, bits, merkleroot, etc.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc. | [required] |
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**block_hash** | **String** | Represents the hash of the block, which is its unique identifier. It represents a cryptographic digital fingerprint made by hashing the block header twice through the SHA256 algorithm. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |

### Return type

[**crate::models::GetBlockDetailsByBlockHashR**](GetBlockDetailsByBlockHashR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_block_details_by_block_height

> crate::models::GetBlockDetailsByBlockHeightR get_block_details_by_block_height(blockchain, network, height, context)
Get Block Details By Block Height

Through this endpoint customers can obtain basic information about a given mined block, specifically by using the `height` parameter. These block details could include the hash of the specific, the previous and the next block, its transactions count, its height, etc.     Blockchain specific data is information such as version, nonce, size, bits, merkleroot, etc.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc. | [required] |
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**height** | **i64** | Represents the number of blocks in the blockchain preceding this specific block. Block numbers have no gaps. A blockchain usually starts with block 0 called the \"Genesis block\". | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |

### Return type

[**crate::models::GetBlockDetailsByBlockHeightR**](GetBlockDetailsByBlockHeightR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_fee_recommendations

> crate::models::GetFeeRecommendationsR get_fee_recommendations(blockchain, network, context)
Get Fee Recommendations

Through this endpoint customers can obtain fee recommendations. Our fees recommendations are based on Mempool data which makes them much more accurate than fees based on already mined blocks. Calculations are done in real time live. Using this endpoint customers can get gas price for Ethereum, fee per byte for Bitcoin, etc.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc. | [required] |
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |

### Return type

[**crate::models::GetFeeRecommendationsR**](GetFeeRecommendationsR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_last_mined_block

> crate::models::GetLastMinedBlockR get_last_mined_block(blockchain, network, context)
Get Last Mined Block

Through this endpoint customers can fetch the last mined block in a specific blockchain network, along with its details. These could include the hash of the specific, the previous and the next block, its transactions count, its height, etc.     Blockchain specific data is information such as version, nonce, size, bits, merkleroot, etc.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc. | [required] |
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |

### Return type

[**crate::models::GetLastMinedBlockR**](GetLastMinedBlockR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_next_available_nonce

> crate::models::GetNextAvailableNonceR get_next_available_nonce(blockchain, network, address, context)
Get Next Available Nonce

Through this endpoint customers can get information about the next available nonce by providing the specific blockchain, network and address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc. | [required] |
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**address** | **String** | Represents the public address, which is a compressed and shortened form of a public key. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |

### Return type

[**crate::models::GetNextAvailableNonceR**](GetNextAvailableNonceR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_raw_transaction_data

> crate::models::GetRawTransactionDataR get_raw_transaction_data(blockchain, network, transaction_id, context)
Get Raw Transaction Data

Through this endpoint customers can get information on a transaction in its raw format by providing its `transactionId`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc. | [required] |
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**transaction_id** | **String** | Represents the unique identifier of a transaction, i.e. it could be transactionId in UTXO-based protocols like Bitcoin, and transaction hash in Ethereum blockchain. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |

### Return type

[**crate::models::GetRawTransactionDataR**](GetRawTransactionDataR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transaction_details_by_transaction_id

> crate::models::GetTransactionDetailsByTransactionIdr get_transaction_details_by_transaction_id(blockchain, network, transaction_id, context)
Get Transaction Details By Transaction ID

Through this endpoint customers can obtain details about a transaction by the transaction's unique identifier. In UTXO-based protocols like BTC there are attributes such as `transactionId` and transaction `hash`. They still could be different. In protocols like Ethereum there is only one unique value and it's `hash`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc. | [required] |
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**transaction_id** | **String** | Represents the unique identifier of a transaction, i.e. it could be `transactionId` in UTXO-based protocols like Bitcoin, and transaction `hash` in Ethereum blockchain. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |

### Return type

[**crate::models::GetTransactionDetailsByTransactionIdr**](GetTransactionDetailsByTransactionIDR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_confirmed_tokens_transfers_by_address_and_time_range

> crate::models::ListConfirmedTokensTransfersByAddressAndTimeRangeR list_confirmed_tokens_transfers_by_address_and_time_range(blockchain, network, address, from_timestamp, to_timestamp, context, limit, offset)
List Confirmed Tokens Transfers By Address And Time Range

Through this endpoint customers can obtain a list with **confirmed** token transfers by the `address` attribute and the query parameters `fromTimestamp` and `toTimestamp` which gives customers the opportunity to filter the results by a specified time period.    {note}This refers only to transfers done for **confirmed tokens** not coins.{/note}

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum, Ethereum Classic, etc. | [required] |
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**address** | **String** | Represents the public address, which is a compressed and shortened form of a public key. | [required] |
**from_timestamp** | **i32** | Defines the specific time/date from which the results will start being listed. | [required] |
**to_timestamp** | **i32** | Defines the specific time/date to which the results will be listed. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |
**limit** | Option<**i64**> | Defines how many items should be returned in the response per page basis. |  |[default to 50]
**offset** | Option<**i64**> | The starting index of the response items, i.e. where the response should start listing the returned items. |  |[default to 0]

### Return type

[**crate::models::ListConfirmedTokensTransfersByAddressAndTimeRangeR**](ListConfirmedTokensTransfersByAddressAndTimeRangeR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_confirmed_transactions_by_address

> crate::models::ListConfirmedTransactionsByAddressR list_confirmed_transactions_by_address(blockchain, network, address, context, limit, offset)
List Confirmed Transactions By Address

This endpoint will list transactions by an attribute `address`. The transactions listed will detail additional information such as hash, height, time of creation in Unix timestamp, etc.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc. | [required] |
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**address** | **String** | Represents the public address, which is a compressed and shortened form of a public key. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |
**limit** | Option<**i64**> | Defines how many items should be returned in the response per page basis. |  |[default to 50]
**offset** | Option<**i64**> | The starting index of the response items, i.e. where the response should start listing the returned items. |  |[default to 0]

### Return type

[**crate::models::ListConfirmedTransactionsByAddressR**](ListConfirmedTransactionsByAddressR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_confirmed_transactions_by_address_and_time_range

> crate::models::ListConfirmedTransactionsByAddressAndTimeRangeR list_confirmed_transactions_by_address_and_time_range(blockchain, network, address, from_timestamp, to_timestamp, context, limit, offset)
List Confirmed Transactions By Address And Time Range

This endpoint will list confirmed transactions by the attribute `address` and the query parameters `fromTimestamp` and `toTimestamp` which gives customers the opportunity to filter the results by a specified time period.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc. | [required] |
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**address** | **String** | Represents the public address, which is a compressed and shortened form of a public key. | [required] |
**from_timestamp** | **i32** | Defines the specific time/date from which the results will start being listed. | [required] |
**to_timestamp** | **i32** | Defines the specific time/date to which the results will be listed. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |
**limit** | Option<**i64**> | Defines how many items should be returned in the response per page basis. |  |[default to 50]
**offset** | Option<**i64**> | The starting index of the response items, i.e. where the response should start listing the returned items. |  |[default to 0]

### Return type

[**crate::models::ListConfirmedTransactionsByAddressAndTimeRangeR**](ListConfirmedTransactionsByAddressAndTimeRangeR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_internal_transactions_by_address_and_time_range

> crate::models::ListInternalTransactionsByAddressAndTimeRangeR list_internal_transactions_by_address_and_time_range(blockchain, network, address, from_timestamp, to_timestamp, context, limit, offset)
List Internal Transactions By Address And Time Range

Through this endpoint customers can list internal transactions by the `address` attribute and the query parameters `fromTimestamp` and `toTimestamp`  which gives customers the opportunity to filter the results by a specified time period.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc. | [required] |
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**address** | **String** | String identifier of the address document represented in CryptoAPIs | [required] |
**from_timestamp** | **i32** | Defines the specific time/date from which the results will start being listed. | [required] |
**to_timestamp** | **i32** | Defines the specific time/date to which the results will be listed. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |
**limit** | Option<**i64**> | Defines how many items should be returned in the response per page basis. |  |[default to 50]
**offset** | Option<**i64**> | The starting index of the response items, i.e. where the response should start listing the returned items. |  |[default to 0]

### Return type

[**crate::models::ListInternalTransactionsByAddressAndTimeRangeR**](ListInternalTransactionsByAddressAndTimeRangeR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_latest_mined_blocks

> crate::models::ListLatestMinedBlocksR list_latest_mined_blocks(network, blockchain, count, context)
List Latest Mined Blocks

Through this endpoint customers can list **up to 50** from the latest blocks that were mined.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks | [required] |
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc. | [required] |
**count** | **i32** | Specifies how many records were requested. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |

### Return type

[**crate::models::ListLatestMinedBlocksR**](ListLatestMinedBlocksR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_transactions_by_block_hash

> crate::models::ListTransactionsByBlockHashR list_transactions_by_block_hash(blockchain, network, block_hash, context, limit, offset)
List Transactions by Block Hash

This endpoint will list transactions by an attribute `transactionHash`. The transactions listed will detail additional information such as addresses, height, time of creation in Unix timestamp, etc.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc. | [required] |
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**block_hash** | **String** | Represents the hash of the block, which is its unique identifier. It represents a cryptographic digital fingerprint made by hashing the block header twice through the SHA256 algorithm. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |
**limit** | Option<**i64**> | Defines how many items should be returned in the response per page basis. |  |[default to 50]
**offset** | Option<**i64**> | The starting index of the response items, i.e. where the response should start listing the returned items. |  |[default to 0]

### Return type

[**crate::models::ListTransactionsByBlockHashR**](ListTransactionsByBlockHashR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_transactions_by_block_height

> crate::models::ListTransactionsByBlockHeightR list_transactions_by_block_height(blockchain, network, height, context, limit, offset)
List Transactions by Block Height

This endpoint will list transactions by an attribute `blockHeight`. The transactions listed will detail additional information such as hash, addresses, time of creation in Unix timestamp, etc.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc. | [required] |
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**height** | **i64** | Represents the number of blocks in the blockchain preceding this specific block. Block numbers have no gaps. A blockchain usually starts with block 0 called the \"Genesis block\". | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |
**limit** | Option<**i64**> | Defines how many items should be returned in the response per page basis. |  |[default to 50]
**offset** | Option<**i64**> | The starting index of the response items, i.e. where the response should start listing the returned items. |  |[default to 0]

### Return type

[**crate::models::ListTransactionsByBlockHeightR**](ListTransactionsByBlockHeightR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_unconfirmed_transactions_by_address

> crate::models::ListUnconfirmedTransactionsByAddressR list_unconfirmed_transactions_by_address(blockchain, network, address, context, limit, offset)
List Unconfirmed Transactions by Address

Through this endpoint customers can list transactions by `address` that are **unconfirmed**.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc. | [required] |
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**address** | **String** | Represents the public address, which is a compressed and shortened form of a public key. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |
**limit** | Option<**i64**> | Defines how many items should be returned in the response per page basis. |  |[default to 50]
**offset** | Option<**i64**> | The starting index of the response items, i.e. where the response should start listing the returned items. |  |[default to 0]

### Return type

[**crate::models::ListUnconfirmedTransactionsByAddressR**](ListUnconfirmedTransactionsByAddressR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_unspent_transaction_outputs_by_address

> crate::models::ListUnspentTransactionOutputsByAddressR list_unspent_transaction_outputs_by_address(blockchain, network, address, context, limit, offset)
List Unspent Transaction Outputs By Address

Through this endpoint customers can list their transactions' unspent outputs by `address`.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc. | [required] |
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**address** | **String** | Represents the address that has unspend funds per which the result is returned. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |
**limit** | Option<**i32**> | Defines how many items should be returned in the response per page basis. |  |[default to 50]
**offset** | Option<**i32**> | The starting index of the response items, i.e. where the response should start listing the returned items. |  |[default to 0]

### Return type

[**crate::models::ListUnspentTransactionOutputsByAddressR**](ListUnspentTransactionOutputsByAddressR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

