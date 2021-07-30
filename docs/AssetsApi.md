# \AssetsApi

All URIs are relative to *https://rest.cryptoapis.io/v2*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_assets_details**](AssetsApi.md#list_assets_details) | **GET** /market-data/assets/details | List Assets Details



## list_assets_details

> crate::models::ListAssetsDetailsR list_assets_details(context, asset_type, crypto_type, limit, offset, waas_enabled)
List Assets Details

This endpoint will return details on a requested asset. The asset could be a cryptocurrency or FIAT asset that we support. Each asset has a unique identifier - `assetId` and a unique symbol in the form of a string, e.g. \"BTC\".    The details returned could include information on the latest rate and rate fluctuation of different periods of time - 24 hours, a week, one hour, the encoding of the logo, and more.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |
**asset_type** | Option<**String**> | Defines the type of the supported asset. This could be either \"crypto\" or \"fiat\". |  |
**crypto_type** | Option<**String**> | Subtype of the crypto assets. Could be COIN or TOKEN |  |
**limit** | Option<**i32**> | Defines how many items should be returned in the response per page basis. |  |[default to 50]
**offset** | Option<**i32**> | The starting index of the response items, i.e. where the response should start listing the returned items. |  |[default to 0]
**waas_enabled** | Option<**bool**> | Show only if WaaS is/not enabled |  |

### Return type

[**crate::models::ListAssetsDetailsR**](ListAssetsDetailsR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

