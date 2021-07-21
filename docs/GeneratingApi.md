# \GeneratingApi

All URIs are relative to *https://rest.cryptoapis.io/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**generate_receiving_address**](GeneratingApi.md#generate_receiving_address) | **POST** /wallet-as-a-service/wallets/{walletId}/{blockchain}/{network}/addresses | Generate Receiving Address



## generate_receiving_address

> crate::models::GenerateReceivingAddressR generate_receiving_address(blockchain, network, wallet_id, context, generate_receiving_address_rb)
Generate Receiving Address

Through this endpoint customers can generate a new Receiving/Deposit Addresses into their Wallet.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc. | [required] |
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\", \"rinkeby\" are test networks. | [required] |
**wallet_id** | **String** | Represents the unique ID of the specific Wallet. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |
**generate_receiving_address_rb** | Option<[**GenerateReceivingAddressRb**](GenerateReceivingAddressRb.md)> |  |  |

### Return type

[**crate::models::GenerateReceivingAddressR**](GenerateReceivingAddressR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

