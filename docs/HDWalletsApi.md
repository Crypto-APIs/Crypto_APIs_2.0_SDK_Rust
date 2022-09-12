# \HDWalletsApi

All URIs are relative to *https://rest.cryptoapis.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**derive_and_sync_new_change_addresses**](HDWalletsApi.md#derive_and_sync_new_change_addresses) | **POST** /blockchain-data/{blockchain}/{network}/hd/derive-sync-change | Derive And Sync New Change Addresses
[**derive_and_sync_new_receiving_addresses**](HDWalletsApi.md#derive_and_sync_new_receiving_addresses) | **POST** /blockchain-data/{blockchain}/{network}/hd/derive-and-sync | Derive And Sync New Receiving Addresses
[**get_hd_wallet__x_pub_y_pub_z_pub_assets**](HDWalletsApi.md#get_hd_wallet__x_pub_y_pub_z_pub_assets) | **GET** /blockchain-data/{blockchain}/{network}/hd/{extendedPublicKey}/assets | Get HD Wallet (xPub, yPub, zPub) Assets
[**get_hd_wallet__x_pub_y_pub_z_pub_details**](HDWalletsApi.md#get_hd_wallet__x_pub_y_pub_z_pub_details) | **GET** /blockchain-data/{blockchain}/{network}/hd/{extendedPublicKey}/details | Get HD Wallet (xPub, yPub, zPub) Details
[**list_hd_wallet__x_pub_y_pub_z_pub_transactions**](HDWalletsApi.md#list_hd_wallet__x_pub_y_pub_z_pub_transactions) | **GET** /blockchain-data/{blockchain}/{network}/hd/{extendedPublicKey}/transactions | List HD Wallet (xPub, yPub, zPub) Transactions
[**list_hd_wallet__x_pub_y_pub_z_pub_utxos**](HDWalletsApi.md#list_hd_wallet__x_pub_y_pub_z_pub_utxos) | **GET** /blockchain-data/{blockchain}/{network}/hd/{extendedPublicKey}/utxos | List HD Wallet (xPub, yPub, zPub) UTXOs
[**list_synced_addresses**](HDWalletsApi.md#list_synced_addresses) | **GET** /blockchain-data/{blockchain}/{network}/hd/{extendedPublicKey}/synced-addresses | List Synced Addresses
[**prepare_a_utxo_based_transaction_from_hd_wallet__x_pub_y_pub_z_pub**](HDWalletsApi.md#prepare_a_utxo_based_transaction_from_hd_wallet__x_pub_y_pub_z_pub) | **POST** /blockchain-data/{blockchain}/{network}/transactions/prepare-utxo-transaction | Prepare A UTXO-Based Transaction From HD Wallet (xPub, yPub, zPub)
[**prepare_an_account_based_transaction_from_hd_wallet__x_pub_y_pub_z_pub**](HDWalletsApi.md#prepare_an_account_based_transaction_from_hd_wallet__x_pub_y_pub_z_pub) | **POST** /blockchain-data/{blockchain}/{network}/transactions/prepare-account-based-transaction | Prepare An Account-Based Transaction From HD Wallet (xPub, yPub, zPub)
[**sync_hd_wallet__x_pub_y_pub_z_pub**](HDWalletsApi.md#sync_hd_wallet__x_pub_y_pub_z_pub) | **POST** /blockchain-data/{blockchain}/{network}/hd/sync | Sync HD Wallet (xPub, yPub, zPub)
[**sync_new_hd_wallet__x_pub_y_pub_z_pub**](HDWalletsApi.md#sync_new_hd_wallet__x_pub_y_pub_z_pub) | **POST** /blockchain-data/{blockchain}/{network}/hd/sync-new | Sync New HD Wallet (xPub, yPub, zPub)



## derive_and_sync_new_change_addresses

> crate::models::DeriveAndSyncNewChangeAddressesR derive_and_sync_new_change_addresses(blockchain, network, context, derive_and_sync_new_change_addresses_rb)
Derive And Sync New Change Addresses

Through this endpoint users can derive 100 change addresses, starting from the last index we have data for, which are then added to the xPub, subscribed for syncing, and start recording data. If no data is available, it will start from index 0.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc. | [required] |
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |
**derive_and_sync_new_change_addresses_rb** | Option<[**DeriveAndSyncNewChangeAddressesRb**](DeriveAndSyncNewChangeAddressesRb.md)> |  |  |

### Return type

[**crate::models::DeriveAndSyncNewChangeAddressesR**](DeriveAndSyncNewChangeAddressesR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## derive_and_sync_new_receiving_addresses

> crate::models::DeriveAndSyncNewReceivingAddressesR derive_and_sync_new_receiving_addresses(blockchain, network, context, derive_and_sync_new_receiving_addresses_rb)
Derive And Sync New Receiving Addresses

Through this endpoint users can derive 100 receiving addresses, starting from the last index we have data for, which are then added to the xPub, subscribed for syncing, and start recording data. If no data is available, it will start from index 0.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc. | [required] |
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |
**derive_and_sync_new_receiving_addresses_rb** | Option<[**DeriveAndSyncNewReceivingAddressesRb**](DeriveAndSyncNewReceivingAddressesRb.md)> |  |  |

### Return type

[**crate::models::DeriveAndSyncNewReceivingAddressesR**](DeriveAndSyncNewReceivingAddressesR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_hd_wallet__x_pub_y_pub_z_pub_assets

> crate::models::GetHdWalletXPubYPubZPubAssetsR get_hd_wallet__x_pub_y_pub_z_pub_assets(blockchain, extended_public_key, network, context, derivation)
Get HD Wallet (xPub, yPub, zPub) Assets

This endpoint will return details on assets we support for a specified from the customer extended public key (xPub). These could be cryptocurrencies, fungible or non-fungible (NFT) tokens. Each asset has a unique identifier - assetId, and a unique symbol in the form of a string, e.g. \"USDT\".

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc. | [required] |
**extended_public_key** | **String** | Defines the account extended publicly known key which is used to derive all child public keys. | [required] |
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |
**derivation** | Option<**String**> | The way how the HD walled derives, for example when the type is ACCOUNT, it derives change and receive addresses while when the type is BIP32 it derives directly. |  |

### Return type

[**crate::models::GetHdWalletXPubYPubZPubAssetsR**](GetHDWalletXPubYPubZPubAssetsR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_hd_wallet__x_pub_y_pub_z_pub_details

> crate::models::GetHdWalletXPubYPubZPubDetailsR get_hd_wallet__x_pub_y_pub_z_pub_details(blockchain, extended_public_key, network, context, derivation)
Get HD Wallet (xPub, yPub, zPub) Details

HD wallet details is useful endpoint to get the most important data about HD wallet without the need to do a lot of calculations, once the HD Wallet is synced using Sync endpoint we keep it up to date and we calculate these details in advance.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc. | [required] |
**extended_public_key** | **String** | Defines the account extended publicly known key which is used to derive all child public keys. | [required] |
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |
**derivation** | Option<**String**> | The way how the HD walled derives, for example when the type is ACCOUNT, it derives change and receive addresses while when the type is BIP32 it derives directly. |  |

### Return type

[**crate::models::GetHdWalletXPubYPubZPubDetailsR**](GetHDWalletXPubYPubZPubDetailsR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_hd_wallet__x_pub_y_pub_z_pub_transactions

> crate::models::ListHdWalletXPubYPubZPubTransactionsR list_hd_wallet__x_pub_y_pub_z_pub_transactions(blockchain, extended_public_key, network, context, derivation, limit, offset)
List HD Wallet (xPub, yPub, zPub) Transactions

This endpoint will list HD Wallet transactions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blockchain** | **String** | Represents the specific blockchain. | [required] |
**extended_public_key** | **String** | Defines the master public key (xPub) of the account. | [required] |
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |
**derivation** | Option<**String**> | The way how the HD walled derives, for example when the type is ACCOUNT, it derives change and receive addresses while when the type is BIP32 it derives directly. |  |
**limit** | Option<**i64**> | Defines how many items should be returned in the response per page basis. |  |[default to 50]
**offset** | Option<**i64**> | The starting index of the response items, i.e. where the response should start listing the returned items. |  |[default to 0]

### Return type

[**crate::models::ListHdWalletXPubYPubZPubTransactionsR**](ListHDWalletXPubYPubZPubTransactionsR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_hd_wallet__x_pub_y_pub_z_pub_utxos

> crate::models::ListHdWalletXPubYPubZPubUtxosR list_hd_wallet__x_pub_y_pub_z_pub_utxos(blockchain, extended_public_key, network, context, derivation, limit, offset)
List HD Wallet (xPub, yPub, zPub) UTXOs

Through this endpoint you can list HD wallet's UTXOs (Unspent Transaction Outputs) by providing extended public key of an already synced HD wallet.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc. | [required] |
**extended_public_key** | **String** | Defines the account extended publicly known key which is used to derive all child public keys. | [required] |
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |
**derivation** | Option<**String**> | The way how the HD walled derives, for example when the type is ACCOUNT, it derives change and receive addresses while when the type is BIP32 it derives directly. |  |
**limit** | Option<**i64**> | Defines how many items should be returned in the response per page basis. |  |[default to 50]
**offset** | Option<**i64**> | The starting index of the response items, i.e. where the response should start listing the returned items. |  |[default to 0]

### Return type

[**crate::models::ListHdWalletXPubYPubZPubUtxosR**](ListHDWalletXPubYPubZPubUTXOsR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_synced_addresses

> crate::models::ListSyncedAddressesR list_synced_addresses(blockchain, extended_public_key, network, context, address_format, is_change_address, limit, offset)
List Synced Addresses

Through this endpoint users can list all addresses that Crypto APIs has synced for a specific xPub. This includes previous and current/new xPubs, what addresses we’ve synced for them, etc.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc. | [required] |
**extended_public_key** | **String** | Defines the account extended publicly known key which is used to derive all child public keys. | [required] |
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |
**address_format** | Option<**String**> | Defines the address format value. |  |
**is_change_address** | Option<**bool**> | Defines if the address is change addres or not. |  |[default to true]
**limit** | Option<**i64**> | Defines how many items should be returned in the response per page basis. |  |[default to 50]
**offset** | Option<**i64**> | The starting index of the response items, i.e. where the response should start listing the returned items. |  |[default to 0]

### Return type

[**crate::models::ListSyncedAddressesR**](ListSyncedAddressesR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## prepare_a_utxo_based_transaction_from_hd_wallet__x_pub_y_pub_z_pub

> crate::models::PrepareAutxoBasedTransactionFromHdWalletXPubYPubZPubR prepare_a_utxo_based_transaction_from_hd_wallet__x_pub_y_pub_z_pub(blockchain, network, context, prepare_autxo_based_transaction_from_hd_wallet_x_pub_y_pub_z_pub_rb)
Prepare A UTXO-Based Transaction From HD Wallet (xPub, yPub, zPub)

Through the “Prepare a UTXO-based transaction from xPub” endpoint users can prepare a transaction for signing from all synced with Crypto APIs addresses for the specific xPub. This is based on the `selectionStrategy` and the addresses’ balances. In the case a user has an address not synced with Crypto APIs, it will not be included. This endpoint applies to all supported UTXO-based blockchain protocols, e.g. Bitcoin, Litecoin, etc.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blockchain** | **String** |  | [required] |
**network** | **String** |  | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |
**prepare_autxo_based_transaction_from_hd_wallet_x_pub_y_pub_z_pub_rb** | Option<[**PrepareAutxoBasedTransactionFromHdWalletXPubYPubZPubRb**](PrepareAutxoBasedTransactionFromHdWalletXPubYPubZPubRb.md)> |  |  |

### Return type

[**crate::models::PrepareAutxoBasedTransactionFromHdWalletXPubYPubZPubR**](PrepareAUTXO-BasedTransactionFromHDWalletXPubYPubZPubR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## prepare_an_account_based_transaction_from_hd_wallet__x_pub_y_pub_z_pub

> crate::models::PrepareAnAccountBasedTransactionFromHdWalletXPubYPubZPubR prepare_an_account_based_transaction_from_hd_wallet__x_pub_y_pub_z_pub(blockchain, network, context, prepare_an_account_based_transaction_from_hd_wallet_x_pub_y_pub_z_pub_rb)
Prepare An Account-Based Transaction From HD Wallet (xPub, yPub, zPub)

Through the “Prepare an account-based transaction from xPub” endpoint users can prepare a transaction for signing from a synced with Crypto APIs address from the specific xPub. This endpoint applies to all supported account-based blockchain protocols, e.g. Ethereum, BSC, etc

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blockchain** | **String** |  | [required] |
**network** | **String** |  | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |
**prepare_an_account_based_transaction_from_hd_wallet_x_pub_y_pub_z_pub_rb** | Option<[**PrepareAnAccountBasedTransactionFromHdWalletXPubYPubZPubRb**](PrepareAnAccountBasedTransactionFromHdWalletXPubYPubZPubRb.md)> |  |  |

### Return type

[**crate::models::PrepareAnAccountBasedTransactionFromHdWalletXPubYPubZPubR**](PrepareAnAccount-BasedTransactionFromHDWalletXPubYPubZPubR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sync_hd_wallet__x_pub_y_pub_z_pub

> crate::models::SyncHdWalletXPubYPubZPubR sync_hd_wallet__x_pub_y_pub_z_pub(blockchain, network, context, sync_hd_wallet_x_pub_y_pub_z_pub_rb)
Sync HD Wallet (xPub, yPub, zPub)

HD wallets usually have a lot of addresses and transactions, getting the data on demand is a heavy operation. That's why we have created this feature, to be able to get HD wallet details or transactions this HD wallet must be synced first. In addition to the initial sync we keep updating the synced HD wallets all the time.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc. | [required] |
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |
**sync_hd_wallet_x_pub_y_pub_z_pub_rb** | Option<[**SyncHdWalletXPubYPubZPubRb**](SyncHdWalletXPubYPubZPubRb.md)> |  |  |

### Return type

[**crate::models::SyncHdWalletXPubYPubZPubR**](SyncHDWalletXPubYPubZPubR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sync_new_hd_wallet__x_pub_y_pub_z_pub

> crate::models::SyncNewHdWalletXPubYPubZPubR sync_new_hd_wallet__x_pub_y_pub_z_pub(blockchain, network, context, sync_new_hd_wallet_x_pub_y_pub_z_pub_rb)
Sync New HD Wallet (xPub, yPub, zPub)

Through this endpoint users can add a brand new xPub to the Crypto APIs system to be ready for deriving. Unlike our other similar endpoint “Sync HD Wallet (xPub, yPub, zPub)”, this endpoint does not create new addresses nor syncs old data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc. | [required] |
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |
**sync_new_hd_wallet_x_pub_y_pub_z_pub_rb** | Option<[**SyncNewHdWalletXPubYPubZPubRb**](SyncNewHdWalletXPubYPubZPubRb.md)> |  |  |

### Return type

[**crate::models::SyncNewHdWalletXPubYPubZPubR**](SyncNewHDWalletXPubYPubZPubR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

