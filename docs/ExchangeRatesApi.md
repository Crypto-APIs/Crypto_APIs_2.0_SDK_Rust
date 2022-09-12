# \ExchangeRatesApi

All URIs are relative to *https://rest.cryptoapis.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_exchange_rate_by_asset_symbols**](ExchangeRatesApi.md#get_exchange_rate_by_asset_symbols) | **GET** /market-data/exchange-rates/by-symbols/{fromAssetSymbol}/{toAssetSymbol} | Get Exchange Rate By Asset Symbols
[**get_exchange_rate_by_assets_ids**](ExchangeRatesApi.md#get_exchange_rate_by_assets_ids) | **GET** /market-data/exchange-rates/by-asset-ids/{fromAssetId}/{toAssetId} | Get Exchange Rate By Assets IDs



## get_exchange_rate_by_asset_symbols

> crate::models::GetExchangeRateByAssetSymbolsR get_exchange_rate_by_asset_symbols(from_asset_symbol, to_asset_symbol, context, calculation_timestamp)
Get Exchange Rate By Asset Symbols

Through this endpoint customers can obtain exchange rates by asset symbols. The process represents the exchange rate value of a single unit of one asset versus another one. Data is provided per unique asset symbol.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from_asset_symbol** | **String** | Defines the base asset symbol to get a rate for. | [required] |
**to_asset_symbol** | **String** | Defines the relation asset symbol in which the base asset rate will be displayed. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |
**calculation_timestamp** | Option<**i32**> | Defines the time of the market data used to calculate the exchange rate in UNIX Timestamp. Oldest possible timestamp is 30 days. |  |

### Return type

[**crate::models::GetExchangeRateByAssetSymbolsR**](GetExchangeRateByAssetSymbolsR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_exchange_rate_by_assets_ids

> crate::models::GetExchangeRateByAssetsIdsR get_exchange_rate_by_assets_ids(from_asset_id, to_asset_id, context, calculation_timestamp)
Get Exchange Rate By Assets IDs

Through this endpoint customers can obtain exchange rates by asset IDs. The process represents the exchange rate value of a single unit of one asset versus another one. Data is provided per unique asset Reference ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from_asset_id** | **String** | Defines the base asset Reference ID to get a rate for. | [required] |
**to_asset_id** | **String** | Defines the relation asset Reference ID in which the base asset rate will be displayed. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |
**calculation_timestamp** | Option<**i32**> | Defines the time of the market data used to calculate the exchange rate in UNIX Timestamp. Oldest possible timestamp is 30 days. |  |

### Return type

[**crate::models::GetExchangeRateByAssetsIdsR**](GetExchangeRateByAssetsIDsR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

