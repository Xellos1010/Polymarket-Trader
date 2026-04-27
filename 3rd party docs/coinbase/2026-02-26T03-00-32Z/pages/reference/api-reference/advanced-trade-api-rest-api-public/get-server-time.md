# get server time

```
curl --request GET \
  --url https://api.coinbase.com/api/v3/brokerage/time \
  --header 'Authorization: Bearer <token>'
```

```
{
  "iso": "<string>",
  "epochSeconds": "<string>",
  "epochMillis": "<string>"
}
```

Get the current time from the Coinbase Advanced API.

```
curl --request GET \
  --url https://api.coinbase.com/api/v3/brokerage/time \
  --header 'Authorization: Bearer <token>'
```

```
{
  "iso": "<string>",
  "epochSeconds": "<string>",
  "epochMillis": "<string>"
}
```

#### Authorizations

A JWT signed using your CDP API Key Secret, encoded in base64. Refer to the [Creating API Keys](https://developer.chrome.com/coinbase-app/authentication-authorization/api-key-authentication) section of our Coinbase App Authentication docs for information on how to generate your Bearer Token.

#### Response

An ISO-8601 representation of the timestamp

A second-precision representation of the timestamp

A millisecond-precision representation of the timestamp