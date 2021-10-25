# \ZilliqaApi

All URIs are relative to *https://rest.cryptoapis.io/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_latest_mined_zilliqa_block**](ZilliqaApi.md#get_latest_mined_zilliqa_block) | **GET** /blockchain-data/zilliqa-specific/{network}/blocks/last | Get Latest Mined Zilliqa Block
[**get_zilliqa_address_details**](ZilliqaApi.md#get_zilliqa_address_details) | **GET** /blockchain-data/zilliqa-specific/{network}/addresses/{address} | Get Zilliqa Address Details
[**get_zilliqa_block_details_by_block_hash**](ZilliqaApi.md#get_zilliqa_block_details_by_block_hash) | **GET** /blockchain-data/zilliqa-specific/{network}/blocks/hash/{blockHash} | Get Zilliqa Block Details By Block Hash
[**get_zilliqa_block_details_by_block_height**](ZilliqaApi.md#get_zilliqa_block_details_by_block_height) | **GET** /blockchain-data/zilliqa-specific/{network}/blocks/height/{blockHeight} | Get Zilliqa Block Details By Block Height
[**get_zilliqa_transaction_details_by_transaction_id**](ZilliqaApi.md#get_zilliqa_transaction_details_by_transaction_id) | **GET** /blockchain-data/zilliqa-specific/{network}/transactions/{transactionHash} | Get Zilliqa Transaction Details by Transaction ID
[**list_zilliqa_transactions_by_address**](ZilliqaApi.md#list_zilliqa_transactions_by_address) | **GET** /blockchain-data/zilliqa-specific/{network}/addresses/{address}/transactions | List Zilliqa Transactions by Address
[**list_zilliqa_transactions_by_block_hash**](ZilliqaApi.md#list_zilliqa_transactions_by_block_hash) | **GET** /blockchain-data/zilliqa-specific/{network}/blocks/hash/{blockHash}/transactions | List Zilliqa Transactions By Block Hash
[**list_zilliqa_transactions_by_block_height**](ZilliqaApi.md#list_zilliqa_transactions_by_block_height) | **GET** /blockchain-data/zilliqa-specific/{network}/blocks/height/{blockHeight}/transactions | List Zilliqa Transactions By Block Height



## get_latest_mined_zilliqa_block

> crate::models::GetLatestMinedZilliqaBlockR get_latest_mined_zilliqa_block(network, context)
Get Latest Mined Zilliqa Block

Through this endpoint users can obtain information on the latest block that has been mined on the Zilliqa blockchain. Data could include the current and previous block hashes, transaction count, and more.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |

### Return type

[**crate::models::GetLatestMinedZilliqaBlockR**](GetLatestMinedZilliqaBlockR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_zilliqa_address_details

> crate::models::GetZilliqaAddressDetailsR get_zilliqa_address_details(network, address, context)
Get Zilliqa Address Details

Through this endpoint customers can obtain information address details from the Zilliqa blockchain.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**address** | **String** | Defines the specific transaction's address. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |

### Return type

[**crate::models::GetZilliqaAddressDetailsR**](GetZilliqaAddressDetailsR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_zilliqa_block_details_by_block_hash

> crate::models::GetZilliqaBlockDetailsByBlockHashR get_zilliqa_block_details_by_block_hash(network, block_hash, context)
Get Zilliqa Block Details By Block Hash

Through this endpoint customers can obtain block details from the Zilliqa blockchain by providing the block Hash parameter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**block_hash** | **String** | Represents the hash of the block, which is its unique identifier. It represents a cryptographic digital fingerprint made by hashing the block header twice through the SHA256 algorithm. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |

### Return type

[**crate::models::GetZilliqaBlockDetailsByBlockHashR**](GetZilliqaBlockDetailsByBlockHashR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_zilliqa_block_details_by_block_height

> crate::models::GetZilliqaBlockDetailsByBlockHeightR get_zilliqa_block_details_by_block_height(network, block_height, context)
Get Zilliqa Block Details By Block Height

Through this endpoint customers can obtain block details from the Zilliqa blockchain by providing the block Height parameter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**block_height** | **i32** | Represents the number of blocks in the blockchain preceding this specific block. Block numbers have no gaps. A blockchain usually starts with block 0 called the \"Genesis block\". | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |

### Return type

[**crate::models::GetZilliqaBlockDetailsByBlockHeightR**](GetZilliqaBlockDetailsByBlockHeightR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_zilliqa_transaction_details_by_transaction_id

> crate::models::GetZilliqaTransactionDetailsByTransactionIdr get_zilliqa_transaction_details_by_transaction_id(network, transaction_hash, context)
Get Zilliqa Transaction Details by Transaction ID

Through this endpoint customers can obtain transaction details on the Zilliqa blockchain by providing a Transaction ID parameter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**transaction_hash** | **String** | String identifier of the transaction | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |

### Return type

[**crate::models::GetZilliqaTransactionDetailsByTransactionIdr**](GetZilliqaTransactionDetailsByTransactionIDR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_zilliqa_transactions_by_address

> crate::models::ListZilliqaTransactionsByAddressR list_zilliqa_transactions_by_address(network, address, context, limit, offset)
List Zilliqa Transactions by Address

Through this endpoint customers can list transactions on the Zilliqa blockchain by the address parameter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**address** | **String** | Defines the specific address of the sender. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |
**limit** | Option<**i32**> | Defines how many items should be returned in the response per page basis. |  |[default to 50]
**offset** | Option<**i32**> | The starting index of the response items, i.e. where the response should start listing the returned items. |  |[default to 0]

### Return type

[**crate::models::ListZilliqaTransactionsByAddressR**](ListZilliqaTransactionsByAddressR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_zilliqa_transactions_by_block_hash

> crate::models::ListZilliqaTransactionsByBlockHashR list_zilliqa_transactions_by_block_hash(network, block_hash, context, limit, offset)
List Zilliqa Transactions By Block Hash

Through this endpoint customers can list transactions on the Zilliqa blockchain by the block hash parameter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**block_hash** | **String** | Represents the hash of the block, which is its unique identifier. It represents a cryptographic digital fingerprint made by hashing the block header twice through the SHA256 algorithm. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |
**limit** | Option<**i32**> | Defines how many items should be returned in the response per page basis. |  |[default to 50]
**offset** | Option<**i32**> | The starting index of the response items, i.e. where the response should start listing the returned items. |  |[default to 0]

### Return type

[**crate::models::ListZilliqaTransactionsByBlockHashR**](ListZilliqaTransactionsByBlockHashR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_zilliqa_transactions_by_block_height

> crate::models::ListZilliqaTransactionsByBlockHeightR list_zilliqa_transactions_by_block_height(network, block_height, context, limit, offset)
List Zilliqa Transactions By Block Height

Through this endpoint customers can list transactions on the Zilliqa blockchain by the block height parameter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**block_height** | **i32** | Represents the number of blocks in the blockchain preceding this specific block. Block numbers have no gaps. A blockchain usually starts with block 0 called the \"Genesis block\". | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |
**limit** | Option<**i32**> | Defines how many items should be returned in the response per page basis. |  |[default to 50]
**offset** | Option<**i32**> | The starting index of the response items, i.e. where the response should start listing the returned items. |  |[default to 0]

### Return type

[**crate::models::ListZilliqaTransactionsByBlockHeightR**](ListZilliqaTransactionsByBlockHeightR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

