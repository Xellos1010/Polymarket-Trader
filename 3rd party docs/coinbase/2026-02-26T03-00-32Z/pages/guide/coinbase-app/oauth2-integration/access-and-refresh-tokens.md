# access and refresh tokens

Coinbase uses an optional security feature of OAuth2 called refresh tokens, if the scope `offline_access` was requested in the authorize (`oauth2/auth`) request. When you first authenticate, your app is given an `access_token` and a `refresh_token`. The access token authenticates all your requests; but **the access token expires in one hour**. Once expired, you must use the refresh token to obtain a new access token and a new refresh token. **The refresh token expires after 1.5 years**; and it can only be exchanged once for a new set of access and refresh tokens. If you try to make a call with an expired access or refresh token, a `401` response is returned.

This process adds some complexity for Coinbase App integrations, but provides an valuable layer of security since a compromised access token is automatically revoked after one hour.

## Refreshing Access & Refresh Tokens

To get a new access token, you must send a POST request to `/oauth2/token` with your `refresh_token` and change the `grant_type` to `refresh_token`.

The `code` and `redirect_uri` parameters are not required for this request.

```
curl https://login.coinbase.com/oauth2/token \
  -X POST \
  -d 'grant_type=refresh_token&
      client_id=YOUR_CLIENT_ID&
      client_secret=YOUR_CLIENT_SECRET&
      refresh_token=REFRESH_TOKEN'

```

The expected result is a response containing the access token, as before:

```
{
  "access_token": "...",
  "token_type": "bearer",
  "expires_in": 3600,
  "refresh_token": "...",
  "scope": "all"
}

```

## Revoking an Access Token

Access tokens can be revoked manually if you want to disconnect your application’s access to the user’s account. Revoking can also be used to implement a log-out feature. You must supply the current access token twice, once to revoke it, and another to authenticate the request (either containing `access_token` parameter or `Authentication` header with bearer token). `200 OK` is returned for both successful and unsuccessful requests.

```
curl https://login.coinbase.com/oauth2/revoke \
  -X POST \
  -d 'token=ACCESS_TOKEN&
      client_id=YOUR_CLIENT_ID&
      client_secret=YOUR_CLIENT_SECRET'
  -H 'Authorization: Bearer 6915ab99857fec1e6f2f6c078583756d0c09d7207750baea28dfbc3d4b0f2cb80'

```