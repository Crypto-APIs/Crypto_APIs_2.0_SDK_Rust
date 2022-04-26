# \FeaturesApi

All URIs are relative to *https://rest.cryptoapis.io/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**broadcast_locally_signed_transaction**](FeaturesApi.md#broadcast_locally_signed_transaction) | **POST** /blockchain-tools/{blockchain}/{network}/transactions/broadcast | Broadcast Locally Signed Transaction
[**decode_raw_transaction_hex**](FeaturesApi.md#decode_raw_transaction_hex) | **POST** /blockchain-tools/{blockchain}/{network}/decode-raw-transaction | Decode Raw Transaction Hex
[**decode_x_address**](FeaturesApi.md#decode_x_address) | **GET** /blockchain-tools/{blockchain}/{network}/decode-x-address/{xAddress} | Decode X-Address
[**derive_hd_wallet__x_pub_y_pub_z_pub_change_or_receiving_addresses**](FeaturesApi.md#derive_hd_wallet__x_pub_y_pub_z_pub_change_or_receiving_addresses) | **GET** /blockchain-tools/{blockchain}/{network}/hd/{extendedPublicKey}/addresses/derive-address | Derive HD Wallet (xPub, yPub, zPub) Change Or Receiving Addresses
[**encode_x_address**](FeaturesApi.md#encode_x_address) | **GET** /blockchain-tools/{blockchain}/{network}/encode-x-address/{classicAddress}/{addressTag} | Encode X-Address
[**estimate_gas_limit**](FeaturesApi.md#estimate_gas_limit) | **POST** /blockchain-tools/{blockchain}/{network}/gas-limit | Estimate Gas Limit
[**estimate_token_gas_limit**](FeaturesApi.md#estimate_token_gas_limit) | **POST** /blockchain-tools/{blockchain}/{network}/gas-limit/contract | Estimate Token Gas Limit
[**get_eip_1559_fee_recommendations**](FeaturesApi.md#get_eip_1559_fee_recommendations) | **GET** /blockchain-tools/{blockchain}/{network}/fees/eip1559 | Get EIP 1559 Fee Recommendations
[**validate_address**](FeaturesApi.md#validate_address) | **POST** /blockchain-tools/{blockchain}/{network}/addresses/validate | Validate Address



## broadcast_locally_signed_transaction

> crate::models::BroadcastLocallySignedTransactionR broadcast_locally_signed_transaction(blockchain, network, context, broadcast_locally_signed_transaction_rb)
Broadcast Locally Signed Transaction

Through this endpoint customers can broadcast transactions that have been already signed locally. Instead of using a node for broadcasting a signed transaction users can use this endpoint. We then keep the user posted about the status by sending you a callback with a success or failure status.    {warning}This can be prepared and signed **only** locally, not through the API. We can provide support only for the process of broadcasting.{/warning}

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc. | [required] |
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |
**broadcast_locally_signed_transaction_rb** | Option<[**BroadcastLocallySignedTransactionRb**](BroadcastLocallySignedTransactionRb.md)> |  |  |

### Return type

[**crate::models::BroadcastLocallySignedTransactionR**](BroadcastLocallySignedTransactionR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## decode_raw_transaction_hex

> crate::models::DecodeRawTransactionHexR decode_raw_transaction_hex(blockchain, network, context, decode_raw_transaction_hex_rb)
Decode Raw Transaction Hex

Through this endpoint customers can decode a raw transaction hex and see the decoded transactions' details.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc. | [required] |
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |
**decode_raw_transaction_hex_rb** | Option<[**DecodeRawTransactionHexRb**](DecodeRawTransactionHexRb.md)> |  |  |

### Return type

[**crate::models::DecodeRawTransactionHexR**](DecodeRawTransactionHexR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## decode_x_address

> crate::models::DecodeXAddressR decode_x_address(blockchain, network, x_address, context)
Decode X-Address

Through this endpoint, customers can decode an encoded XRP address with tag, by providing the specific x-address. The response includes the decoded classic address and the tag.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc. | [required] |
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**x_address** | **String** | Represents the encoded classic address with its destination tag. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |

### Return type

[**crate::models::DecodeXAddressR**](DecodeX-AddressR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## derive_hd_wallet__x_pub_y_pub_z_pub_change_or_receiving_addresses

> crate::models::DeriveHdWalletXPubYPubZPubChangeOrReceivingAddressesR derive_hd_wallet__x_pub_y_pub_z_pub_change_or_receiving_addresses(blockchain, extended_public_key, network, context, address_format, addresses_count, is_change, start_index)
Derive HD Wallet (xPub, yPub, zPub) Change Or Receiving Addresses

Through this endpoint, customers can derive up to 10 addresses - both change and receive, from a certain HD Wallet (xPub, yPub, zPub), by providing an extended public key. By default the system creates a receiving/deposit address, unless the isChange attribute is set to 'true'. In that case the system derives a 'change' address. The change address can be derived only for UTXO based blockchains, for all the rest, this endpoint always creates a deposit/receiving address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc. | [required] |
**extended_public_key** | **String** | Defines the account extended publicly known key which is used to derive all child public keys. | [required] |
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |
**address_format** | Option<**String**> | Represents the format of the address. |  |
**addresses_count** | Option<**i32**> | Represents the addresses count. |  |
**is_change** | Option<**bool**> | Defines if the specific address is a change or deposit address. If the value is True - it is a change address, if it is False - it is a Deposit address. |  |
**start_index** | Option<**i32**> | The starting index of the response items, i.e. where the response should start listing the returned items. |  |

### Return type

[**crate::models::DeriveHdWalletXPubYPubZPubChangeOrReceivingAddressesR**](DeriveHDWalletXPubYPubZPubChangeOrReceivingAddressesR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## encode_x_address

> crate::models::EncodeXAddressR encode_x_address(address_tag, blockchain, classic_address, network, context)
Encode X-Address

Through this endpoint, customers can encode an encoded XRP address with tag, by providing the specific x-address. The response includes the encoded classic address and the tag.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address_tag** | **i32** | Defines a specific Tag that is an additional XRP address feature. It helps identifying a transaction recipient beyond a wallet address. | [required] |
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc. | [required] |
**classic_address** | **String** | Represents the public address, which is a compressed and shortened form of a public key. | [required] |
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |

### Return type

[**crate::models::EncodeXAddressR**](EncodeX-AddressR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## estimate_gas_limit

> crate::models::EstimateGasLimitR estimate_gas_limit(blockchain, network, context, estimate_gas_limit_rb)
Estimate Gas Limit

This endpoint helps customer in estimating the gas limit needed for a transaction. It gives information for gas expenses when sending ether to contracts or making a transaction with additional data in it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum. | [required] |
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |
**estimate_gas_limit_rb** | Option<[**EstimateGasLimitRb**](EstimateGasLimitRb.md)> |  |  |

### Return type

[**crate::models::EstimateGasLimitR**](EstimateGasLimitR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## estimate_token_gas_limit

> crate::models::EstimateTokenGasLimitR estimate_token_gas_limit(blockchain, network, context, estimate_token_gas_limit_rb)
Estimate Token Gas Limit

This endpoint helps customer in estimating the Contract Gas Limit needed for a transaction. It gives information for gas expenses for a specific contract when sending ethers or making a transaction with additional data in it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum. | [required] |
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |
**estimate_token_gas_limit_rb** | Option<[**EstimateTokenGasLimitRb**](EstimateTokenGasLimitRb.md)> |  |  |

### Return type

[**crate::models::EstimateTokenGasLimitR**](EstimateTokenGasLimitR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_eip_1559_fee_recommendations

> crate::models::GetEip1559FeeRecommendationsR get_eip_1559_fee_recommendations(network, blockchain, context)
Get EIP 1559 Fee Recommendations

Through this endpoint customers can obtain fee recommendations specifically for EIP 1559.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |

### Return type

[**crate::models::GetEip1559FeeRecommendationsR**](GetEIP1559FeeRecommendationsR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## validate_address

> crate::models::ValidateAddressR validate_address(blockchain, network, context, validate_address_rb)
Validate Address

This endpoint checks user public addresses whether they are valid or not.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc. | [required] |
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |
**validate_address_rb** | Option<[**ValidateAddressRb**](ValidateAddressRb.md)> |  |  |

### Return type

[**crate::models::ValidateAddressR**](ValidateAddressR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

