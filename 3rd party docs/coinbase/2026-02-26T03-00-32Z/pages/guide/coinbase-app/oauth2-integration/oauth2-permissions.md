# Coinbase App OAuth2 Scopes - Coinbase Developer Documentation

Scopes allow you to specify fine-grained access for your OAuth2 applications. Getting your scopes right is key to developing safe and trustworthy applications.

## Overview

With OAuth2, scopes are set in the [authorization URL](https://developer.chrome.com/coinbase-app/oauth2-integration/integrations) and determine what API endpoints your application can access. All authenticated endpoints, except `GET /user`, require a specific scope. With OAuth2, scopes should be considered as **grants**—users can select which scopes they allow for your application. To see which scopes a user has granted, use the `GET /user/auth` endpoint.

### Naming pattern

Scopes follow the pattern `service:resource:action`. The main services are `wallet` and `data`.

Action

Description

`read`

List or read resources (e.g., listing transactions)

`create`

Create new resources (some have specific actions like `send`)

`update`

Update existing resources

`delete`

Delete resources

## Account access

In addition to scopes, Coinbase App applications can request different levels of access to user’s wallets. This access is defined by a dropdown selection on the consent page when the user connects to your app. ![Account access consent screen](https://mintcdn.com/coinbase-prod/Nwe99SJqMKlRkcv5/coinbase-app/images/account-access.png?fit=max&auto=format&n=Nwe99SJqMKlRkcv5&q=85&s=4da570748bb56dbb358a193b864600e2) Account access works **together with scopes**. For example, `account=all` combined with `scope=wallet:buys:create` allows your app to create buys on all of the user’s wallets, but won’t give access to sell on any of their accounts.

## Specifying scopes

Scopes are specified by including a `scope` parameter in your OAuth2 authorization request. Multiple scopes should be separated with a comma:

```
https://login.coinbase.com/oauth2/auth?response_type=code&client_id=YOUR_CLIENT_ID&redirect_uri=YOUR_CALLBACK_URL&scope=wallet:accounts:read,wallet:transactions:read

```

If you need to obtain more permissions later, you can re-authenticate the user, prompting them to authorize additional scopes.

## Supported scopes

Below are listed all the available scopes for both Coinbase App application and API keys. For more information to understand which permission is required for a specific API action/endpoint, follow our `API reference` which includes *Permissions* section under each endpoint.

Scope

Description

`wallet:accounts:read`

List user’s accounts and their balances

`wallet:accounts:update`

Update account (e.g. change name)

`wallet:accounts:create`

Create a new account (e.g. BTC wallet)

`wallet:accounts:delete`

Delete existing account

`wallet:addresses:read`

List account’s bitcoin or ethereum addresses

`wallet:addresses:create`

Create new bitcoin or ethereum addresses for wallets

`wallet:buys:read`

List account’s buys

`wallet:buys:create`

Buy bitcoin or ethereum

`wallet:deposits:read`

List account’s deposits

`wallet:deposits:create`

Create a new deposit

`wallet:notifications:read`

List user’s notifications

`wallet:payment-methods:read`

List user’s payment methods (e.g. bank accounts)

`wallet:payment-methods:delete`

Remove existing payment methods

`wallet:payment-methods:limits`

Get detailed limits for payment methods (useful for performing buys and sells). This permission is to be used together with `wallet:payment-methods:read`

`wallet:sells:read`

List account’s sells

`wallet:sells:create`

Sell bitcoin or ethereum

`wallet:trades:read`

List trades

`wallet:trades:create`

Create trades

`wallet:transactions:read`

List account’s transactions

`wallet:transactions:send`

Send bitcoin or ethereum

`wallet:transactions:request`

Request bitcoin or ethereum from a Coinbase user

`wallet:transactions:transfer`

Transfer funds between user’s two bitcoin or ethereum accounts

`wallet:user:read`

List detailed user information (public information is available without this permission)

`wallet:user:update`

Update current user

`wallet:user:email`

Read current user’s email address

`wallet:withdrawals:read`

List account’s withdrawals

`wallet:withdrawals:create`

Create a new withdrawal

`offline_access`

Return a refresh token in response