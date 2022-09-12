# \TransactionsApi

All URIs are relative to *https://rest.cryptoapis.io*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_coins_transaction_from_address_for_whole_amount**](TransactionsApi.md#create_coins_transaction_from_address_for_whole_amount) | **POST** /wallet-as-a-service/wallets/{walletId}/{blockchain}/{network}/addresses/{address}/all-transaction-requests | Create Coins Transaction From Address For Whole Amount
[**create_coins_transaction_request_from_address**](TransactionsApi.md#create_coins_transaction_request_from_address) | **POST** /wallet-as-a-service/wallets/{walletId}/{blockchain}/{network}/addresses/{address}/transaction-requests | Create Coins Transaction Request from Address
[**create_coins_transaction_request_from_wallet**](TransactionsApi.md#create_coins_transaction_request_from_wallet) | **POST** /wallet-as-a-service/wallets/{walletId}/{blockchain}/{network}/transaction-requests | Create Coins Transaction Request from Wallet
[**create_fungible_token_transaction_request_from_address_without_fee_priority**](TransactionsApi.md#create_fungible_token_transaction_request_from_address_without_fee_priority) | **POST** /wallet-as-a-service/wallets/{walletId}/{blockchain}/{network}/addresses/{senderAddress}/feeless-token-transaction-requests | Create Fungible Token Transaction Request From Address Without Fee Priority
[**create_fungible_tokens_transaction_request_from_address**](TransactionsApi.md#create_fungible_tokens_transaction_request_from_address) | **POST** /wallet-as-a-service/wallets/{walletId}/{blockchain}/{network}/addresses/{senderAddress}/token-transaction-requests | Create Fungible Tokens Transaction Request from Address
[**create_single_transaction_request_from_address_without_fee_priority**](TransactionsApi.md#create_single_transaction_request_from_address_without_fee_priority) | **POST** /wallet-as-a-service/wallets/{walletId}/{blockchain}/{network}/addresses/{address}/feeless-transaction-requests | Create Single Transaction Request From Address Without Fee Priority



## create_coins_transaction_from_address_for_whole_amount

> crate::models::CreateCoinsTransactionFromAddressForWholeAmountR create_coins_transaction_from_address_for_whole_amount(address, blockchain, network, wallet_id, context, create_coins_transaction_from_address_for_whole_amount_rb)
Create Coins Transaction From Address For Whole Amount

Through this endpoint customers can create a new transaction from address for **coins** specifically, which will transfer over the entire available amount.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | Defines the source address. | [required] |
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc. | [required] |
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**wallet_id** | **String** | Represents the sender's specific and unique Wallet ID of the sender. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |
**create_coins_transaction_from_address_for_whole_amount_rb** | Option<[**CreateCoinsTransactionFromAddressForWholeAmountRb**](CreateCoinsTransactionFromAddressForWholeAmountRb.md)> |  |  |

### Return type

[**crate::models::CreateCoinsTransactionFromAddressForWholeAmountR**](CreateCoinsTransactionFromAddressForWholeAmountR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_coins_transaction_request_from_address

> crate::models::CreateCoinsTransactionRequestFromAddressR create_coins_transaction_request_from_address(address, blockchain, network, wallet_id, context, create_coins_transaction_request_from_address_rb)
Create Coins Transaction Request from Address

Through this endpoint users can create a new single transaction request from one address to another.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | Defines the specific source address for the transaction. For XRP we also support the X-address format. | [required] |
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc. | [required] |
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**wallet_id** | **String** | Represents the sender's specific and unique Wallet ID of the sender. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |
**create_coins_transaction_request_from_address_rb** | Option<[**CreateCoinsTransactionRequestFromAddressRb**](CreateCoinsTransactionRequestFromAddressRb.md)> |  |  |

### Return type

[**crate::models::CreateCoinsTransactionRequestFromAddressR**](CreateCoinsTransactionRequestFromAddressR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_coins_transaction_request_from_wallet

> crate::models::CreateCoinsTransactionRequestFromWalletR create_coins_transaction_request_from_wallet(blockchain, network, wallet_id, context, create_coins_transaction_request_from_wallet_rb)
Create Coins Transaction Request from Wallet

Through this endpoint users can create a new transaction request from the entire Wallet instead from just a specific address. This endpoint can generate transactions from multiple to multiple addresses.    {warning}This is available **only** for UTXO-based protocols such as Bitcoin, Bitcoin Cash, Litecoin, etc. It **is not** available for Account-based protocols like Ethereum.{/warning}

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc. | [required] |
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |[default to testnet]
**wallet_id** | **String** | Represents the sender's specific and unique Wallet ID of the sender. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |
**create_coins_transaction_request_from_wallet_rb** | Option<[**CreateCoinsTransactionRequestFromWalletRb**](CreateCoinsTransactionRequestFromWalletRb.md)> |  |  |

### Return type

[**crate::models::CreateCoinsTransactionRequestFromWalletR**](CreateCoinsTransactionRequestFromWalletR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_fungible_token_transaction_request_from_address_without_fee_priority

> crate::models::CreateFungibleTokenTransactionRequestFromAddressWithoutFeePriorityR create_fungible_token_transaction_request_from_address_without_fee_priority(blockchain, network, sender_address, wallet_id, context, create_fungible_token_transaction_request_from_address_without_fee_priority_rb)
Create Fungible Token Transaction Request From Address Without Fee Priority

Through this endpoint customers can make a single feeless token transaction on the Tron blockchain protocol. TRX transactions burn certain resources called Bandwidth and Energy. Each account has 1500 bandwidth free for use every 24 hours and more can be obtained by staking TRX. The unit price of Energy is 280 SUN and of bandwidth - 1000 SUN. If the resources are insufficient, TRX will be burned to pay for them.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc. | [required] |
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**sender_address** | **String** | Defines the specific source address for the transaction. | [required] |
**wallet_id** | **String** | Defines the unique ID of the Wallet. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |
**create_fungible_token_transaction_request_from_address_without_fee_priority_rb** | Option<[**CreateFungibleTokenTransactionRequestFromAddressWithoutFeePriorityRb**](CreateFungibleTokenTransactionRequestFromAddressWithoutFeePriorityRb.md)> |  |  |

### Return type

[**crate::models::CreateFungibleTokenTransactionRequestFromAddressWithoutFeePriorityR**](CreateFungibleTokenTransactionRequestFromAddressWithoutFeePriorityR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_fungible_tokens_transaction_request_from_address

> crate::models::CreateFungibleTokensTransactionRequestFromAddressR create_fungible_tokens_transaction_request_from_address(blockchain, network, sender_address, wallet_id, context, create_fungible_tokens_transaction_request_from_address_rb)
Create Fungible Tokens Transaction Request from Address

Through this endpoint users can make a single token transaction.    {note}To have an operational callback subscription, you need to first verify a domain for the Callback URL. Please see more information on Callbacks [here](https://developers.cryptoapis.io/technical-documentation/general-information/callbacks#callback-url).{/note}    {warning}Crypto APIs will notify the user **only when** the event occurs. There are cases when the specific event doesn't happen at all, or takes a long time to do so. A callback notification **will not** be sent if the event does not or cannot occur, or will take long time to occur.{/warning}

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc. | [required] |[default to ethereum]
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |[default to mainnet]
**sender_address** | **String** | Defines the specific source address for the transaction. | [required] |
**wallet_id** | **String** | Defines the unique ID of the Wallet. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |
**create_fungible_tokens_transaction_request_from_address_rb** | Option<[**CreateFungibleTokensTransactionRequestFromAddressRb**](CreateFungibleTokensTransactionRequestFromAddressRb.md)> |  |  |

### Return type

[**crate::models::CreateFungibleTokensTransactionRequestFromAddressR**](CreateFungibleTokensTransactionRequestFromAddressR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_single_transaction_request_from_address_without_fee_priority

> crate::models::CreateSingleTransactionRequestFromAddressWithoutFeePriorityR create_single_transaction_request_from_address_without_fee_priority(address, blockchain, network, wallet_id, context, create_single_transaction_request_from_address_without_fee_priority_rb)
Create Single Transaction Request From Address Without Fee Priority

Through this endpoint users can create a new single transaction request from one address to another. The difference between this endpoint and \"Create Coins Transaction Request from Address\"  is that for Tron blockchain there is no Fee Priority that defines how fast a transaction can be mined.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | Defines the specific source address for the transaction. | [required] |
**blockchain** | **String** | Represents the specific blockchain protocol name, e.g. Ethereum, Bitcoin, etc. | [required] |
**network** | **String** | Represents the name of the blockchain network used; blockchain networks are usually identical as technology and software, but they differ in data, e.g. - \"mainnet\" is the live network with actual data while networks like \"testnet\", \"ropsten\" are test networks. | [required] |
**wallet_id** | **String** | Represents the sender's specific and unique Wallet ID of the sender. | [required] |
**context** | Option<**String**> | In batch situations the user can use the context to correlate responses with requests. This property is present regardless of whether the response was successful or returned as an error. `context` is specified by the user. |  |
**create_single_transaction_request_from_address_without_fee_priority_rb** | Option<[**CreateSingleTransactionRequestFromAddressWithoutFeePriorityRb**](CreateSingleTransactionRequestFromAddressWithoutFeePriorityRb.md)> |  |  |

### Return type

[**crate::models::CreateSingleTransactionRequestFromAddressWithoutFeePriorityR**](CreateSingleTransactionRequestFromAddressWithoutFeePriorityR.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

