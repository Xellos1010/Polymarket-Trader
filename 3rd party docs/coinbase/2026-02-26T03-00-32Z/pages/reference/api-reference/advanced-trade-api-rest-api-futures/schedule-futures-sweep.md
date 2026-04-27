# schedule futures sweep

```
curl --request POST \
  --url https://api.coinbase.com/api/v3/brokerage/cfm/sweeps/schedule \
  --header 'Authorization: Bearer <token>' \
  --header 'Content-Type: application/json' \
  --data '
{
  "usd_amount": "<string>"
}
'
```

Schedules a sweep of funds from FCM wallet to USD Spot wallet

POST

/

api

/

v3

/

brokerage

/

cfm

/

sweeps

/

schedule

```
curl --request POST \
  --url https://api.coinbase.com/api/v3/brokerage/cfm/sweeps/schedule \
  --header 'Authorization: Bearer <token>' \
  --header 'Content-Type: application/json' \
  --data '
{
  "usd_amount": "<string>"
}
'
```

#### Authorizations

A JWT signed using your CDP API Key Secret, encoded in base64. Refer to the [Creating API Keys](https://developer.chrome.com/coinbase-app/authentication-authorization/api-key-authentication) section of our Coinbase App Authentication docs for information on how to generate your Bearer Token.

#### Body

The amount of USD to be swept. By default, sweeps all available excess funds.

#### Response