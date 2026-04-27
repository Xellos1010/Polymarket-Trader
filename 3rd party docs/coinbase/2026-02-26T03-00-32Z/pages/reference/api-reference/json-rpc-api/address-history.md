# address history

## Constructing Requests

**Option 1**: You can use the [JSON RPC playground](https://portal.cdp.coinbase.com/products/onchain-data) to easily create sample JSON RPC requests. **Option 2**: Alternatively, you can construct your own requests by following the below steps:

1.  Log into the [CDP Portal](https://portal.cdp.coinbase.com/) and go to the [Data](https://portal.cdp.coinbase.com/products/onchain-data) page.
2.  Ensure the correct network in your RPC URL based on the [supported RPC networks](https://developer.chrome.com/api-reference/networks#cdp-features).
3.  Open a terminal and make your requests based on the supported methods in this API reference page.

### `cdp_listBalances`

This endpoint retrieves the latest balances for an address. Note that this JSON RPC endpoint provides a more complete list of assets than the [CDP API](https://developer.chrome.com/api-reference/rest-api/addresses/list-address-balances). However, there is a few seconds of delay in terms of data freshness for indexing a complete list of assets. If you need more real-time data with whitelisted assets, you should use the [CDP API](https://developer.chrome.com/api-reference/rest-api/addresses/list-address-balances). The [API credit](https://developer.chrome.com/data/node/overview#rate-limits) value of this method is 100.

#### Parameters

Name

Type

Req

Description

address

string

Y

Blockchain address hash. EVM chain address hash should be lowercase.

pageSize

string

N

Number of balances to receive in a page. The default value is 25. The maximum value is 100, and values supplied over this will be coerced to the maximum.

pageToken

string

N

Provided from a previous response’s nextPageToken

#### Returns

Field

Description

asset.id

The identity of the asset for querying for details or history.

asset.type

The type of Asset the definition describes. For example: “native”, “erc20”, “erc721”, “erc1155”, “creditAlphanum4”, “fa2”.

asset.groupId

The contract address or group identifier for an Asset. For an NFT or a multi-token Asset this may identify a group of Asset. For a native Asset this will not be set.

asset.subGroupId

The identifier that distinguishes the identity of the Asset within the contract address or group. For an NFT or a multi-token Asset that can have many Assets associated with a contract address, this could be a token ID. For a UTXO, this could be the coin identifier.

value

The amount of the balance in the lowest denomination of the asset. Type is in BigInteger in standard base64 encoding.

valueStr

The string representation of the balance value to avoid precision loss.

decimals

The number of decimals the asset utilizes.

nextPageToken

A token which can be provided as `pageToken` to retrieve the next page. If this field is omitted, there are no additional pages.

#### Request/Response

### `cdp_listBalanceDetails`

This endpoint lists the latest balance details for an asset for an address. Similar to `cdp_listBalances`, if you need more real-time data, you should use the [CDP API](https://developer.chrome.com/api-reference/rest-api/addresses/list-address-balances). The [API credit](https://developer.chrome.com/data/node/overview#rate-limits) value of this method is 100.

#### Parameters

Name

Type

Req

Description

address

string

Y

Blockchain address hash. EVM chain address hash should be lowercase.

assetId

string

Y

Provided from ListBalances or ListAddressTransactions response’s asset field

pageSize

string

N

Number of balances to receive in a page. The default value is 25. The maximum value is 100, and values supplied over this will be coerced to the maximum.

pageToken

string

N

Provided from a previous response’s nextPageToken

#### Returns

Field

Description

asset.id

The identity of the asset for querying for details or history.

asset.type

The type of Asset the definition describes. For example: “native”, “erc20”, “erc721”, “erc1155”, “creditAlphanum4”, “fa2”.

asset.groupId

The contract address or group identifier for an Asset. For an NFT or a multi-token Asset this may identify a group of Asset. For a native Asset this will not be set.

asset.subGroupId

The identifier that distinguishes the identity of the Asset within the contract address or group. For an NFT or a multi-token Asset that can have many Assets associated with a contract address, this could be a token ID. For a UTXO, this could be the coin identifier.

value

The amount of the balance in the lowest denomination of the asset. Type is in BigInteger in standard base64 encoding.

valueStr

The string representation of the balance value to avoid precision loss.

decimals

The number of decimals the asset utilizes.

nextPageToken

A token which can be provided as `pageToken` to retrieve the next page. If this field is omitted, there are no additional pages.

#### Request/Response

### `cdp_listBalanceHistories`

This endpoint lists the balance histories for an asset for an address. `cdp_listBalanceHistory` is also a valid method name for this method. The [API credit](https://developer.chrome.com/data/node/overview#rate-limits) value of this method is 100.

#### Parameters

Name

Type

Req

Description

address

string

Y

Blockchain address hash. EVM chain address hash should be lowercase.

assetId

string

Y

Provided from ListBalances or ListAddressTransactions response’s asset field

pageSize

string

N

Number of balances to receive in a page. The default value is 25. The maximum value is 100, and values supplied over this will be coerced to the maximum.

pageToken

string

N

Provided from a previous response’s nextPageToken

#### Returns

Field

Description

blockHash

The hash of the block this transaction was included in.

blockHeight

The height of the block this transaction was included in.

value

The amount of the balance in the lowest denomination of the asset. Type is in BigInteger in standard base64 encoding.

valueStr

The string representation of the balance value to avoid precision loss.

nextPageToken

A token which can be provided as `pageToken` to retrieve the next page. If this field is omitted, there are no additional pages.

#### Request/Response

### `cdp_listAddressTransactions`

This endpoint lists the transactions for an address. `cdp_listTransactions` is also a valid method name for this method. The [API credit](https://developer.chrome.com/data/node/overview#rate-limits) value of this method is 100.

#### Parameters

Name

Type

Req

Description

address

string

Y

Blockchain address hash. EVM chain address hash should be lowercase.

pageSize

string

N

Number of balances to receive in a page. The default value is 25. The maximum value is 100, and values supplied over this will be coerced to the maximum.

pageToken

string

N

Provided from a previous response’s nextPageToken

#### Returns

Field

Description

name

A unique identifier for the transaction, which cannot be changed once created.In the format of `networks/{network}/indexers/{indexer}/transactions/{transaction}`

hash

The transaction hash

blockHash

The hash of the block this transaction was included in.

blockHeight

The height of the block this transaction was included in.

status

The status of the transaction

content

The transaction content in either ethereum or rosetta format.

#### Request/Response