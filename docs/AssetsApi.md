# \AssetsApi

All URIs are relative to *https://rest.cryptoapis.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_asset_details_by_asset_id**](AssetsApi.md#get_asset_details_by_asset_id) | **GET** /market-data/assets/assetId/{assetId} | Get Asset Details By Asset ID
[**get_asset_details_by_asset_symbol**](AssetsApi.md#get_asset_details_by_asset_symbol) | **GET** /market-data/assets/{assetSymbol} | Get Asset Details By Asset Symbol



## get_asset_details_by_asset_id

> crate::models::GetAssetDetailsByAssetIdr get_asset_details_by_asset_id(asset_id, context)
Get Asset Details By Asset ID

Through this endpoint users can obtain information on assets by `assetId`.    The details returned could include information on the latest rate and rate fluctuation of different periods of time - 24 hours, a week, one hour, the encoding of the logo, and more.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_id** | **String** | Defines the unique ID of the specific asset. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |

### Return type

[**crate::models::GetAssetDetailsByAssetIdr**](GetAssetDetailsByAssetIDR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_asset_details_by_asset_symbol

> crate::models::GetAssetDetailsByAssetSymbolR get_asset_details_by_asset_symbol(asset_symbol, context)
Get Asset Details By Asset Symbol

Through this endpoint users can obtain information on assets by asset symbol.    The details returned could include information on the latest rate and rate fluctuation of different periods of time - 24 hours, a week, one hour, the encoding of the logo, and more.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_symbol** | **String** | Specifies the asset's unique symbol in the Crypto APIs listings. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |

### Return type

[**crate::models::GetAssetDetailsByAssetSymbolR**](GetAssetDetailsByAssetSymbolR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

