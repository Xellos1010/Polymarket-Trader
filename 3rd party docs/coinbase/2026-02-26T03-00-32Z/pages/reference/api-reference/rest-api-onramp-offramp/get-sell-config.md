# get sell config

```
curl --request GET \
  --url https://api.developer.coinbase.com/onramp/v1/sell/config \
  --header 'Authorization: Bearer <token>'
```

```
{
  "countries": [
    {
      "id": "<string>",
      "payment_methods": [
        {
          "id": "UNSPECIFIED"
        }
      ],
      "subdivisions": [
        "<string>"
      ]
    }
  ]
}
```

The Sell Config API returns the list of countries supported by Coinbase Pay Offramp, and the fiat deposit methods available in each country. Clients should call this API periodically and cache the response.

```
curl --request GET \
  --url https://api.developer.coinbase.com/onramp/v1/sell/config \
  --header 'Authorization: Bearer <token>'
```

```
{
  "countries": [
    {
      "id": "<string>",
      "payment_methods": [
        {
          "id": "UNSPECIFIED"
        }
      ],
      "subdivisions": [
        "<string>"
      ]
    }
  ]
}
```

#### Authorizations

Enter your JSON Web Token (JWT) here. Refer to the [Generate JWT](https://developer.chrome.com/api-reference/authentication#2-generate-jwt-server-only) section of our Authentication docs for information on how to generate your Bearer Token.

#### Response

List of supported countries and fiat deposit methods for selling