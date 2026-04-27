# available apis

## Overview

You can access these APIs after a user authorizes your app via OAuth2. Each endpoint requires specific [OAuth2 scopes](https://developer.chrome.com/coinbase-app/oauth2-integration/scopes). **Base URL:** `https://api.coinbase.com` **Authentication:** Include the OAuth2 access token in the Authorization header:

```
curl https://api.coinbase.com/v2/<endpoint> \
  -H "Authorization: Bearer YOUR_ACCESS_TOKEN"

```

## Accounts and users

Endpoint

Description

Scope

`GET /v2/user`

[Get current user’s public information](https://developer.chrome.com/coinbase-app/track-apis/accounts)

`wallet:user:read`

`GET /v2/accounts`

[List all user accounts and balances](https://developer.chrome.com/coinbase-app/track-apis/accounts#list-accounts)

`wallet:accounts:read`

`GET /v2/accounts/:account_id`

[Get a specific account](https://developer.chrome.com/coinbase-app/track-apis/accounts#show-account)

`wallet:accounts:read`

## Transactions

Endpoint

Description

Scope

`GET /v2/accounts/:account_id/transactions`

[List transactions](https://developer.chrome.com/coinbase-app/track-apis/transactions#list-transactions)

`wallet:transactions:read`

`GET /v2/accounts/:account_id/transactions/:id`

[Get a specific transaction](https://developer.chrome.com/coinbase-app/track-apis/transactions#show-transaction)

`wallet:transactions:read`

`POST /v2/accounts/:account_id/transactions`

[Send crypto (requires 2FA)](https://developer.chrome.com/coinbase-app/transfer-apis/send-crypto)

`wallet:transactions:send`

## Addresses

Endpoint

Description

Scope

`GET /v2/accounts/:account_id/addresses`

[List addresses](https://developer.chrome.com/coinbase-app/transfer-apis/onchain-addresses)

`wallet:addresses:read`

`POST /v2/accounts/:account_id/addresses`

[Create a new address](https://developer.chrome.com/coinbase-app/transfer-apis/onchain-addresses)

`wallet:addresses:create`

## Deposits & withdrawals

Endpoint

Description

Scope

`GET /v2/accounts/:account_id/deposits`

[List deposits](https://developer.chrome.com/coinbase-app/transfer-apis/deposit-fiat#list-deposits)

`wallet:deposits:read`

`GET /v2/accounts/:account_id/deposits/:deposit_id`

[Show deposit](https://developer.chrome.com/coinbase-app/transfer-apis/deposit-fiat#show-deposit)

`wallet:deposits:read`

`POST /v2/accounts/:account_id/deposits`

[Deposit fiat funds](https://developer.chrome.com/coinbase-app/transfer-apis/deposit-fiat#deposit-funds)

`wallet:deposits:create`

`POST /v2/accounts/:account_id/deposits/:deposit_id/commit`

[Commit deposit](https://developer.chrome.com/coinbase-app/transfer-apis/deposit-fiat#commit-deposit)

`wallet:deposits:create`

`GET /v2/accounts/:account_id/withdrawals`

[List withdrawals](https://developer.chrome.com/coinbase-app/transfer-apis/withdraw-fiat#list-withdrawals)

`wallet:withdrawals:read`

`GET /v2/accounts/:account_id/withdrawals/:withdrawal_id`

[Show withdrawal](https://developer.chrome.com/coinbase-app/transfer-apis/withdraw-fiat#show-withdrawal)

`wallet:withdrawals:read`

`POST /v2/accounts/:account_id/withdrawals`

[Withdraw fiat funds](https://developer.chrome.com/coinbase-app/transfer-apis/withdraw-fiat#withdraw-funds)

`wallet:withdrawals:create`

`POST /v2/accounts/:account_id/withdrawals/:withdrawal_id/commit`

[Commit withdrawal](https://developer.chrome.com/coinbase-app/transfer-apis/withdraw-fiat#commit-withdrawal)

`wallet:withdrawals:create`

## Advanced Trade APIs

For trading functionality, you can also access [Advanced Trade APIs](https://developer.chrome.com/coinbase-app/advanced-trade-apis/overview) with OAuth2 tokens. See the [OAuth2 Access Guide](https://developer.chrome.com/coinbase-app/advanced-trade-apis/guides/oauth-access) for details on portfolio access.

Endpoint

Description

Scope

`GET /api/v3/brokerage/accounts`

[List trading accounts](https://developer.chrome.com/coinbase-app/advanced-trade-apis/rest-api)

[See guide](https://developer.chrome.com/coinbase-app/advanced-trade-apis/guides/oauth-access)

`POST /api/v3/brokerage/orders`

[Create orders](https://developer.chrome.com/coinbase-app/advanced-trade-apis/rest-api)

[See guide](https://developer.chrome.com/coinbase-app/advanced-trade-apis/guides/oauth-access)

`GET /api/v3/brokerage/orders/historical/batch`

[List orders](https://developer.chrome.com/coinbase-app/advanced-trade-apis/rest-api)

[See guide](https://developer.chrome.com/coinbase-app/advanced-trade-apis/guides/oauth-access)

## Scopes reference

For a complete list of OAuth2 scopes and what they enable, see the [Scopes Reference](https://developer.chrome.com/coinbase-app/oauth2-integration/scopes).