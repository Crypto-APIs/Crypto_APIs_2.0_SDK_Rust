# \DefaultApi

All URIs are relative to *https://rest.cryptoapis.io/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_contract_details_by_address**](DefaultApi.md#get_contract_details_by_address) | **get** /blockchain-data/{blockchain}/{network}/addresses/{contractAddress}/contract | Get Contract Details by Address



## get_contract_details_by_address

> crate::models::GetContractDetailsByAddressResponse get_contract_details_by_address(blockchain, network, contract_address, context)
Get Contract Details by Address

This endpoint will return a smart contract details by address, this address is the address of the smart contract. It's not the same as the smart contract creator address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blockchain** | **String** |  | [required] |
**network** | **String** |  | [required] |
**contract_address** | **String** | String identifier of the token | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |

### Return type

[**crate::models::GetContractDetailsByAddressResponse**](GetContractDetailsByAddressResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

