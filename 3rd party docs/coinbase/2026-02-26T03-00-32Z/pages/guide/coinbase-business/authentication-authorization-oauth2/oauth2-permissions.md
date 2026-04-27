# oauth2 permissions

Different applications require different access to user accounts, and Coinbase Business provides many options to fine-tune the access. Options range from accounts to API endpoints accessed by API consumer. For full list of options, see [OAuth2 reference](https://developer.chrome.com/coinbase-business/authentication-authorization/oauth2/reference).

## Account Access

Coinbase Business applications can request different access to user’s wallets. This access is defined by a drop down selection on the consent page when the user connects to the app. See the example below. ![](https://mintcdn.com/coinbase-prod/Nwe99SJqMKlRkcv5/coinbase-app/images/account-access.png?fit=max&auto=format&n=Nwe99SJqMKlRkcv5&q=85&s=4da570748bb56dbb358a193b864600e2) Note that Wallet access is still used together with OAuth2 scopes (see below). This means that `account=all` combined with `scope=wallet:buys:create` can create buys on all of user’s wallets but won’t for example give access to sell on any of their accounts.

## OAuth2 Permission Scopes

For OAuth2, permissions are specified by including an additional `scope` parameter in your OAuth2 request. For example, your app may only need to view a user’s accounts and transaction history, but may not need or want the ability to send/receive and buy/sell a [digital asset](https://help.coinbase.com/en/coinbase/supported-crypto). Multiple permissions should be separated with a comma character in the URL (i.e. `&scope=wallet:accounts:read,wallet:transactions:read`). It’s recommended that you only ask for permissions that your application needs. If you need to obtain more permissions later, you can re-authenticate the user, forcing the user to consider authorizing additional permissions the next time s/he opens the app. Here is an example request URL with a `scope` parameter on the end:

```
https://login.coinbase.com/oauth2/auth?response_type=code&client_id=YOUR_CLIENT_ID&redirect_uri=YOUR_CALLBACK_URL&scope=wallet:accounts:read,wallet:transactions:read

```

[Full list of permission (scopes)](https://developer.chrome.com/coinbase-app/authentication-authorization/oauth2/scopes)