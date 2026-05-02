# delete portfolio

```
curl --request DELETE \
  --url https://api.coinbase.com/api/v3/brokerage/portfolios/{portfolio_uuid} \
  --header 'Authorization: Bearer <token>'
```

Delete portfolio.

DELETE

/

api

/

v3

/

brokerage

/

portfolios

/

{portfolio\_uuid}

```
curl --request DELETE \
  --url https://api.coinbase.com/api/v3/brokerage/portfolios/{portfolio_uuid} \
  --header 'Authorization: Bearer <token>'
```

#### Authorizations

A JWT signed using your CDP API Key Secret, encoded in base64. Refer to the [Creating API Keys](https://developer.chrome.com/coinbase-app/authentication-authorization/api-key-authentication) section of our Coinbase App Authentication docs for information on how to generate your Bearer Token.

#### Path Parameters

#### Response

The response is of type `object`.