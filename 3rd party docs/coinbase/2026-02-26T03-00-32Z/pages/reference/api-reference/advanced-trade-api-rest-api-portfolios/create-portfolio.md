# create portfolio

```
curl --request POST \
  --url https://api.coinbase.com/api/v3/brokerage/portfolios \
  --header 'Authorization: Bearer <token>' \
  --header 'Content-Type: application/json' \
  --data '
{
  "name": "<string>"
}
'
```

```
{
  "portfolio": {
    "name": "<string>",
    "uuid": "<string>",
    "type": "UNDEFINED",
    "deleted": true
  }
}
```

Create a portfolio.

POST

/

api

/

v3

/

brokerage

/

portfolios

```
curl --request POST \
  --url https://api.coinbase.com/api/v3/brokerage/portfolios \
  --header 'Authorization: Bearer <token>' \
  --header 'Content-Type: application/json' \
  --data '
{
  "name": "<string>"
}
'
```

```
{
  "portfolio": {
    "name": "<string>",
    "uuid": "<string>",
    "type": "UNDEFINED",
    "deleted": true
  }
}
```

#### Authorizations

A JWT signed using your CDP API Key Secret, encoded in base64. Refer to the [Creating API Keys](https://developer.chrome.com/coinbase-app/authentication-authorization/api-key-authentication) section of our Coinbase App Authentication docs for information on how to generate your Bearer Token.

#### Body

The name of the portfolio.

#### Response

Portfolio is the identifying information for a portfolio.