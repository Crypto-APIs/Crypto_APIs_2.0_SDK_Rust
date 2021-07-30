# \GeneratingApi

All URIs are relative to *https://rest.cryptoapis.io/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**generate_deposit_address**](GeneratingApi.md#generate_deposit_address) | **POST** /wallet-as-a-service/wallets/{walletId}/{blockchain}/{network}/addresses | Generate Deposit Address



## generate_deposit_address

> crate::models::GenerateDepositAddressR generate_deposit_address(blockchain, network, wallet_id, context, generate_deposit_address_rb)
Generate Deposit Address

Through this endpoint customers can generate a new Receiving/Deposit Addresses into their Wallet.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc. | [required] |
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\", \"rinkeby\" are test networks. | [required] |
**wallet_id** | **String** | Represents the unique ID of the specific Wallet. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |
**generate_deposit_address_rb** | Option<[**GenerateDepositAddressRb**](GenerateDepositAddressRb.md)> |  |  |

### Return type

[**crate::models::GenerateDepositAddressR**](GenerateDepositAddressR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

