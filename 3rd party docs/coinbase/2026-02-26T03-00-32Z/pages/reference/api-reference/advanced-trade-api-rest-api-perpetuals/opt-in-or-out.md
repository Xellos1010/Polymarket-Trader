# opt in or out

Opt In or Out of Multi Asset Collateral

```
curl --request POST \
  --url https://api.coinbase.com/api/v3/brokerage/intx/multi_asset_collateral \
  --header 'Authorization: Bearer <token>' \
  --header 'Content-Type: application/json' \
  --data '
{
  "portfolio_uuid": "<string>",
  "multi_asset_collateral_enabled": true
}
'
```

```
{
  "multi_asset_collateral_enabled": true
}
```

International Derivatives

Enable or Disable Multi Asset Collateral for a given Portfolio

POST

/

api

/

v3

/

brokerage

/

intx

/

multi\_asset\_collateral

Opt In or Out of Multi Asset Collateral

```
curl --request POST \
  --url https://api.coinbase.com/api/v3/brokerage/intx/multi_asset_collateral \
  --header 'Authorization: Bearer <token>' \
  --header 'Content-Type: application/json' \
  --data '
{
  "portfolio_uuid": "<string>",
  "multi_asset_collateral_enabled": true
}
'
```

```
{
  "multi_asset_collateral_enabled": true
}
```

#### Authorizations

A JWT signed using your CDP API Key Secret, encoded in base64. Refer to the [Creating API Keys](https://developer.chrome.com/coinbase-app/authentication-authorization/api-key-authentication) section of our Coinbase App Authentication docs for information on how to generate your Bearer Token.

#### Body

multi\_asset\_collateral\_enabled

Enable or disable Multi Asset Collateral.

#### Response

multi\_asset\_collateral\_enabled