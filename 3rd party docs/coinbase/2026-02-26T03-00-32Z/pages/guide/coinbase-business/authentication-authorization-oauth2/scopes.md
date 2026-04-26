# scopes

Permissions, also known as scopes with OAuth2, allow you to specify fine-grained access for your Coinbase Business (OAuth2) applications and API keys. Getting your permissions right is the key to developing safe and trustworthy applications on Coinbase’s APIs. With OAuth2, permissions are set in the [authorization URL](https://developer.chrome.com/coinbase-app/authentication-authorization/oauth2/integrations). Some permissions, like sending funds, also requires additional settings. Unlike Coinbase Business permissions, API key permissions are defined when creating an API key and can be changes afterwards. All authenticated endpoints, except `GET /user`, require a specific scope to access them. In general, permissions follow the pattern,`service-name:resource:action`, where the main services are `wallet` and `data`. In general, most common actions are:

-   `read` - List or read individual resources (e.g., listing your transactions)
-   `create` - Create new resources. For some resources there’s more specific actions (e.g. `send` and `refund`)
-   `update` - Update existing resource
-   `delete` - Delete a resource

With OAuth2, permissions should be considered as grants: Users can select which permissions (scopes) they grant access to for the application. The application might need to request new scopes over the lifecycle of the authorization. To see which permissions the user has granted, you can use `GET /user/auth` endpoint. As a general rule, you should only ask for scopes which your application needs and avoid asking for access to unnecessary ones. Users more readily grant access to limited, clearly described scopes.

## Supported Scopes

Below are listed all the available scopes for both Coinbase Business application and API keys. For more information to understand which permission is required for a specific API action/endpoint, follow our `API reference` which includes *Permissions* section under each endpoint.

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