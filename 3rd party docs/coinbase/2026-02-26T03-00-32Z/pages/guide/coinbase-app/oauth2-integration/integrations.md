# integrations

## Registering OAuth2 Client

Before integrating Coinbase App, you need to register a new OAuth2 application under your [CDP API settings](https://portal.cdp.coinbase.com/projects/api-keys/oauth). If you’re using a previous OAuth2 implementation, you may need to configure the following settings:

-   **Authorize URL**: `https://login.coinbase.com/oauth2/auth`
-   **Access Token URL**: `https://login.coinbase.com/oauth2/token`

## Integrating with OAuth2 Library

Coinbase recommends integrating your OAuth client with a battle-tested OAuth2 library so that you can simply plug in your client id, client secret, etc. Consider using one of the following [OAuth2 libraries](https://oauth.net/code/). For enhanced security, consider implementing [PKCE (Proof Key for Code Exchange)](https://developer.chrome.com/coinbase-app/oauth2-integration/security-best-practices#pkce-proof-key-for-code-exchange) in your OAuth2 flow.

## Integrating Manually

To integrate your third-party web server application with Coinbase, use the following flow:

### 1\. Redirect users to request Coinbase access

```
GET https://login.coinbase.com/oauth2/auth?response_type=code&client_id=YOUR_CLIENT_ID&redirect_uri=YOUR_REDIRECT_URL&state=SECURE_RANDOM&scope=wallet:accounts:read+offline_access

```

When redirecting a user to Coinbase to authorize access to your application, you’ll need to construct the authorization URL with the correct parameters and scopes. Here’s a list of parameters you should always specify:

Parameter

Description

`response_type`

**Required** Value `code`

`client_id`

**Required** The client ID you received after registering your application.

`redirect_uri`

**Optional** The URL in your app where users will be sent after authorization (see below). This value needs to be URL encoded. If left out, your application’s first redirect URI will be used by default.

`state`

**Optional** An unguessable random string to protect against cross-site request forgery attacks. Must be at least 8 characters long. [Read more](https://developer.chrome.com/coinbase-app/oauth2-integration/security-best-practices)

`scope`

**Optional** Comma separated list of permissions (scopes) your application requests access to. Required scopes are listed under endpoints in the [Scopes reference](https://developer.chrome.com/coinbase-app/oauth2-integration/scopes)

`code_challenge`

**Optional** PKCE code challenge for additional security. If provided, `code_verifier` must be included in the token exchange step. [Read more](https://developer.chrome.com/coinbase-app/oauth2-integration/security-best-practices#pkce-proof-key-for-code-exchange)

`code_challenge_method`

**Optional** Method used to generate the code challenge. Supported values: `S256` (recommended) or `plain`. Defaults to `plain` if not specified.

Example of an authorization URL:

```
GET https://login.coinbase.com/oauth2/auth?response_type=code&client_id=1532c63424622b6e9c4654e7f97ed40194a1547e114ca1c682f44283f39dfa49&redirect_uri=https%3A%2F%2Fexample.com%2Foauth%2Fcallback&state=134ef5504a94&scope=wallet:user:read,wallet:accounts:read

```

You can further customize the authorization page and permissions. Read more at [OAuth2 reference](https://developer.chrome.com/coinbase-app/oauth2-integration/reference).

### 2\. Coinbase redirects back to your site

If the user approves your application, Coinbase will redirect them back to your `redirect_uri` with a temporary `code` parameter. If you specified a `state` parameter in step 1, it will be returned as well. The parameter will always match the value specified in step 1. If the values don’t match, the request should not be trusted. Example of the redirect:

```
GET https://example.com/oauth/callback?code=4c666b5c0c0d9d3140f2e0776cbe245f3143011d82b7a2c2a590cc7e20b79ae8&state=134ef5504a94

```

### 3\. Exchange `code` for an access token

After you have received the temporary `code`, you can exchange it for valid access and refresh tokens. This can be done by making a POST call:

```
POST https://login.coinbase.com/oauth2/token

```

With following parameters:

Parameter

Description

`grant_type`

**Required** Value `authorization_code`

`code`

**Required** Value from step 2

`client_id`

**Required** The client ID you received after registering your application.

`client_secret`

**Required** The client secret you received after registering your application.

`redirect_uri`

**Required** Your application’s redirect URI

`code_verifier`

**Required if `code_challenge` was used** PKCE code verifier that corresponds to the code challenge sent in step 1.

Example request:

```
curl https://login.coinbase.com/oauth2/token \
  -X POST \
  -d 'grant_type=authorization_code&code=4c666b5c0c0d9d3140f2e0776cbe245f3143011d82b7a2c2a590cc7e20b79ae8&client_id=1532c63424622b6e9c4654e7f97ed40194a1547e114ca1c682f44283f39dfa49&client_secret=3a21f08c585df35c14c0c43b832640b29a3a3a18e5c54d5401f08c87c8be0b20&redirect_uri=https://example.com/oauth/callback'

```

Example request with PKCE:

```
curl https://login.coinbase.com/oauth2/token \
  -X POST \
  -d 'grant_type=authorization_code&code=4c666b5c0c0d9d3140f2e0776cbe245f3143011d82b7a2c2a590cc7e20b79ae8&client_id=1532c63424622b6e9c4654e7f97ed40194a1547e114ca1c682f44283f39dfa49&client_secret=3a21f08c585df35c14c0c43b832640b29a3a3a18e5c54d5401f08c87c8be0b20&redirect_uri=https://example.com/oauth/callback&code_verifier=YOUR_CODE_VERIFIER'

```

After a successful request, a valid access token will be returned in the response:

```
{
  "access_token": "6915ab99857fec1e6f2f6c078583756d0c09d7207750baea28dfbc3d4b0f2cb80",
  "token_type": "bearer",
  "expires_in": 3600,
  "refresh_token": "73a3431906de603504c1e8437709b0f47d07bed11981fe61b522278a81a9232b7",
  "scope": "wallet:user:read wallet:accounts:read"
}

```

### 4\. Make an API call

After you have a valid access token, you can make your first API call:

```
curl https://api.coinbase.com/v2/user /
  -H 'Authorization: Bearer 6915ab99857fec1e6f2f6c078583756d0c09d7207750baea28dfbc3d4b0f2cb80'

```

Example response:

```
{
  "data": {
    "id": "9da7a204-544e-5fd1-9a12-61176c5d4cd8",
    "name": "User One",
    "username": "user1",
    "profile_location": null,
    "profile_bio": null,
    "profile_url": "https://coinbase.com/user1",
    "avatar_url": "https://images.coinbase.com/avatar?h=vR%2FY8igBoPwuwGren5JMwvDNGpURAY%2F0nRIOgH%2FY2Qh%2BQ6nomR3qusA%2Bh6o2%0Af9rH&s=128",
    "resource": "user",
    "resource_path": "/v2/user"
  }
}

```