# \ManageSubscriptionsApi

All URIs are relative to *https://rest.cryptoapis.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**activate_blockchain_event_subscription**](ManageSubscriptionsApi.md#activate_blockchain_event_subscription) | **POST** /blockchain-events/subscriptions/{referenceId}/activate | Activate Blockchain Event Subscription
[**delete_blockchain_event_subscription**](ManageSubscriptionsApi.md#delete_blockchain_event_subscription) | **DELETE** /blockchain-events/{blockchain}/{network}/subscriptions/{referenceId} | Delete Blockchain Event Subscription
[**get_blockchain_event_subscription_details_by_reference_id**](ManageSubscriptionsApi.md#get_blockchain_event_subscription_details_by_reference_id) | **GET** /blockchain-events/subscriptions/{referenceId} | Get Blockchain Event Subscription Details By Reference ID
[**list_blockchain_events_subscriptions**](ManageSubscriptionsApi.md#list_blockchain_events_subscriptions) | **GET** /blockchain-events/{blockchain}/{network}/subscriptions | List Blockchain Events Subscriptions



## activate_blockchain_event_subscription

> crate::models::ActivateBlockchainEventSubscriptionR activate_blockchain_event_subscription(reference_id, context, activate_blockchain_event_subscription_rb)
Activate Blockchain Event Subscription

Through this endpoint customers can reactivate an event subscription (callback) which has been deactivated by the system. Deactivations could happen due to various reasons, most often \"maximum retry attempts reached\".

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reference_id** | **String** | Represents a unique ID used to reference the specific callback subscription. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |
**activate_blockchain_event_subscription_rb** | Option<[**ActivateBlockchainEventSubscriptionRb**](ActivateBlockchainEventSubscriptionRb.md)> |  |  |

### Return type

[**crate::models::ActivateBlockchainEventSubscriptionR**](ActivateBlockchainEventSubscriptionR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_blockchain_event_subscription

> crate::models::DeleteBlockchainEventSubscriptionR delete_blockchain_event_subscription(blockchain, network, reference_id, context)
Delete Blockchain Event Subscription

Through this endpoint the customer can delete blockchain event subscriptions they have by attributes `referenceId` and `network`.    Currently Crypto APIs 2.0 offers certain Blockchain event endpoints which allow the user to subscribe for one/a few/all and receive callback notifications when the specific event occurs.    {note}To have an operational callback subscription, you need to first verify a domain for the Callback URL. Please see more information on Callbacks [here](https://developers.cryptoapis.io/technical-documentation/general-information/callbacks#callback-url).{/note}    {warning}Crypto APIs will notify the user **only when** the event occurs. There are cases when the specific event doesn't happen at all, or takes a long time to do so. A callback notification **will not** be sent if the event does not or cannot occur, or will take long time to occur.{/warning}

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc. | [required] |
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**reference_id** | **String** | Represents a unique ID used to reference the specific callback subscription. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |

### Return type

[**crate::models::DeleteBlockchainEventSubscriptionR**](DeleteBlockchainEventSubscriptionR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_blockchain_event_subscription_details_by_reference_id

> crate::models::GetBlockchainEventSubscriptionDetailsByReferenceIdr get_blockchain_event_subscription_details_by_reference_id(reference_id, context)
Get Blockchain Event Subscription Details By Reference ID

Through this endpoint the customer can get detailed information for a callback subscription by providing its reference ID.    Currently Crypto APIs 2.0 offers certain Blockchain event endpoints which allow the user to subscribe for one/a few/all and receive callback notifications when the specific event occurs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reference_id** | **String** | Represents a unique ID used to reference the specific callback subscription. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |

### Return type

[**crate::models::GetBlockchainEventSubscriptionDetailsByReferenceIdr**](GetBlockchainEventSubscriptionDetailsByReferenceIDR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_blockchain_events_subscriptions

> crate::models::ListBlockchainEventsSubscriptionsR list_blockchain_events_subscriptions(blockchain, network, context, limit, offset)
List Blockchain Events Subscriptions

Through this endpoint the customer can obtain a list of their callback subscriptions for the available Blockchain events.    Currently Crypto APIs 2.0 offers certain Blockchain event endpoints which allow the user to subscribe for one/a few/all and receive callback notifications when the specific event occurs.    {note}To have an operational callback subscription, you need to first verify a domain for the Callback URL. Please see more information on Callbacks [here](https://developers.cryptoapis.io/technical-documentation/general-information/callbacks#callback-url).{/note}

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc. | [required] |
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |
**limit** | Option<**i64**> | Defines how many items should be returned in the response per page basis. |  |[default to 50]
**offset** | Option<**i64**> | The starting index of the response items, i.e. where the response should start listing the returned items. |  |[default to 0]

### Return type

[**crate::models::ListBlockchainEventsSubscriptionsR**](ListBlockchainEventsSubscriptionsR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

