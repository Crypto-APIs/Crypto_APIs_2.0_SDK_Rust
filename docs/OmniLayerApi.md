# \OmniLayerApi

All URIs are relative to *https://rest.cryptoapis.io/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_omni_transaction_details_by_transaction_id__txid**](OmniLayerApi.md#get_omni_transaction_details_by_transaction_id__txid) | **GET** /blockchain-data/{blockchain}/{network}/omni/transactions/{transactionId} | Get Omni Transaction Details By Transaction ID (Txid)
[**get_unconfirmed_omni_transaction_by_transaction_id__txid**](OmniLayerApi.md#get_unconfirmed_omni_transaction_by_transaction_id__txid) | **GET** /blockchain-data/{blockchain}/{network}/omni/transactions-unconfirmed/{transactionId} | Get Unconfirmed Omni Transaction By Transaction ID (Txid)
[**list_omni_tokens_by_address**](OmniLayerApi.md#list_omni_tokens_by_address) | **GET** /blockchain-data/{blockchain}/{network}/omni/addresses/{address} | List Omni Tokens By Address
[**list_omni_transactions_by_address**](OmniLayerApi.md#list_omni_transactions_by_address) | **GET** /blockchain-data/{blockchain}/{network}/omni/addresses/{address}/transactions | List Omni Transactions By Address
[**list_omni_transactions_by_block_hash**](OmniLayerApi.md#list_omni_transactions_by_block_hash) | **GET** /blockchain-data/{blockchain}/{network}/omni/blocks/hash/{blockHash}/transactions | List Omni Transactions By Block Hash
[**list_omni_transactions_by_block_height**](OmniLayerApi.md#list_omni_transactions_by_block_height) | **GET** /blockchain-data/{blockchain}/{network}/omni/blocks/height/{blockHeight}/transactions | List Omni Transactions By Block Height
[**list_unconfirmed_omni_transactions_by_address**](OmniLayerApi.md#list_unconfirmed_omni_transactions_by_address) | **GET** /blockchain-data/{blockchain}/{network}/omni/address-transactions-unconfirmed/{address} | List Unconfirmed Omni Transactions By Address
[**list_unconfirmed_omni_transactions_by_property_id**](OmniLayerApi.md#list_unconfirmed_omni_transactions_by_property_id) | **GET** /blockchain-data/{blockchain}/{network}/omni/properties/{propertyId}/transactions | List Unconfirmed Omni Transactions By Property ID



## get_omni_transaction_details_by_transaction_id__txid

> crate::models::GetOmniTransactionDetailsByTransactionIdTxidR get_omni_transaction_details_by_transaction_id__txid(network, blockchain, transaction_id, context)
Get Omni Transaction Details By Transaction ID (Txid)

Through this endpoint customers can obtain details about an Omni transaction by the transaction's unique identifier. The transaction can return information such as hash, height, time of creation in Unix timestamp, etc.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc. | [required] |
**transaction_id** | **String** | Represents the unique identifier of a transaction, i.e. it could be `transactionId` in UTXO-based protocols like Bitcoin, and transaction `hash` in Ethereum blockchain. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |

### Return type

[**crate::models::GetOmniTransactionDetailsByTransactionIdTxidR**](GetOmniTransactionDetailsByTransactionIDTxidR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_unconfirmed_omni_transaction_by_transaction_id__txid

> crate::models::GetUnconfirmedOmniTransactionByTransactionIdTxidR get_unconfirmed_omni_transaction_by_transaction_id__txid(network, blockchain, transaction_id, context)
Get Unconfirmed Omni Transaction By Transaction ID (Txid)

Through this endpoint customers can obtain information on unconfirmed Omni transactions by an attribute `transactionId`. The transaction can have information such as hash, height, time of creation in Unix timestamp, etc.    Unconfirmed transactions are usually put in the Mempool and await verification so that they can be added to a block.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc. | [required] |
**transaction_id** | **String** | Represents the unique identifier of a transaction, i.e. it could be `transactionId` in UTXO-based protocols like Bitcoin, and transaction `hash` in Ethereum blockchain. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |

### Return type

[**crate::models::GetUnconfirmedOmniTransactionByTransactionIdTxidR**](GetUnconfirmedOmniTransactionByTransactionIDTxidR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_omni_tokens_by_address

> crate::models::ListOmniTokensByAddressR list_omni_tokens_by_address(network, blockchain, address, context)
List Omni Tokens By Address

Through this endpoint the customer can receive basic information about a given Omni address based on confirmed/synced blocks only. In the case where there are any incoming or outgoing **unconfirmed** transactions for the specific address, they **will not** be counted or calculated here.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc. | [required] |
**address** | **String** | Represents the public address, which is a compressed and shortened form of a public key. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |

### Return type

[**crate::models::ListOmniTokensByAddressR**](ListOmniTokensByAddressR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_omni_transactions_by_address

> crate::models::ListOmniTransactionsByAddressR list_omni_transactions_by_address(network, blockchain, address, context, limit, offset)
List Omni Transactions By Address

This endpoint will list Omni transactions by an attribute `address`. The transactions listed will detail additional information such as hash, height, time of creation in Unix timestamp, etc.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc. | [required] |
**address** | **String** | Represents the public address, which is a compressed and shortened form of a public key. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |
**limit** | Option<**i32**> | Defines how many items should be returned in the response per page basis. |  |[default to 50]
**offset** | Option<**i32**> | The starting index of the response items, i.e. where the response should start listing the returned items. |  |[default to 0]

### Return type

[**crate::models::ListOmniTransactionsByAddressR**](ListOmniTransactionsByAddressR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_omni_transactions_by_block_hash

> crate::models::ListOmniTransactionsByBlockHashR list_omni_transactions_by_block_hash(network, blockchain, block_hash, context, limit, offset)
List Omni Transactions By Block Hash

This endpoint will list Omni transactions by an attribute `transactionHash`. The transactions listed will detail additional information such as addresses, height, time of creation in Unix timestamp, etc.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc. | [required] |
**block_hash** | **String** | Represents the hash of the block, which is its unique identifier. It represents a cryptographic digital fingerprint made by hashing the block header twice through the SHA256 algorithm. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |
**limit** | Option<**i32**> | Defines how many items should be returned in the response per page basis. |  |[default to 50]
**offset** | Option<**i32**> | The starting index of the response items, i.e. where the response should start listing the returned items. |  |[default to 0]

### Return type

[**crate::models::ListOmniTransactionsByBlockHashR**](ListOmniTransactionsByBlockHashR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_omni_transactions_by_block_height

> crate::models::ListOmniTransactionsByBlockHeightR list_omni_transactions_by_block_height(network, blockchain, block_height, context, limit, offset)
List Omni Transactions By Block Height

This endpoint will list Omni transactions by an attribute `blockHeight`. The transactions listed will detail additional information such as hash, addresses, time of creation in Unix timestamp, etc.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc. | [required] |
**block_height** | **String** | Represents the number of blocks in the blockchain preceding this specific block. Block numbers have no gaps. A blockchain usually starts with block 0 called the \"Genesis block\". | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |
**limit** | Option<**i32**> | Defines how many items should be returned in the response per page basis. |  |[default to 50]
**offset** | Option<**i32**> | The starting index of the response items, i.e. where the response should start listing the returned items. |  |[default to 0]

### Return type

[**crate::models::ListOmniTransactionsByBlockHeightR**](ListOmniTransactionsByBlockHeightR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_unconfirmed_omni_transactions_by_address

> crate::models::ListUnconfirmedOmniTransactionsByAddressR list_unconfirmed_omni_transactions_by_address(network, blockchain, address, context, limit, offset)
List Unconfirmed Omni Transactions By Address

This endpoint will list unconfirmed Omni transactions by an attribute `address`. The transactions listed will detail additional information such as hash, height, time of creation in Unix timestamp, etc.    Unconfirmed transactions are usually put in the Mempool and await verification so that they can be added to a block.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc. | [required] |
**address** | **String** | Represents the public address, which is a compressed and shortened form of a public key. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |
**limit** | Option<**i32**> | Defines how many items should be returned in the response per page basis. |  |[default to 50]
**offset** | Option<**i32**> | The starting index of the response items, i.e. where the response should start listing the returned items. |  |[default to 0]

### Return type

[**crate::models::ListUnconfirmedOmniTransactionsByAddressR**](ListUnconfirmedOmniTransactionsByAddressR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_unconfirmed_omni_transactions_by_property_id

> crate::models::ListUnconfirmedOmniTransactionsByPropertyIdr list_unconfirmed_omni_transactions_by_property_id(network, blockchain, property_id, context, limit, offset)
List Unconfirmed Omni Transactions By Property ID

This endpoint will list unconfirmed Omni transactions by an attribute `propertyId`. The transactions listed will detail additional information such as hash, height, time of creation in Unix timestamp, etc.    Unconfirmed transactions are usually put in the Mempool and await verification so that they can be added to a block.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc. | [required] |
**property_id** | **String** | Represents the identifier of the tokens to send. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |
**limit** | Option<**i32**> | Defines how many items should be returned in the response per page basis. |  |[default to 50]
**offset** | Option<**i32**> | The starting index of the response items, i.e. where the response should start listing the returned items. |  |[default to 0]

### Return type

[**crate::models::ListUnconfirmedOmniTransactionsByPropertyIdr**](ListUnconfirmedOmniTransactionsByPropertyIDR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

