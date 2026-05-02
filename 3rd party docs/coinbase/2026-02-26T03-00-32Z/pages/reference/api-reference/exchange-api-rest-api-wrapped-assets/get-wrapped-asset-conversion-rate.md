# get wrapped asset conversion rate

Get wrapped asset conversion rate

### Testing

You can test the `cbETH` conversion rate by sending an HTTP GET request to the following URL: [https://api.exchange.coinbase.com/wrapped-assets/CBETH/conversion-rate](https://api.exchange.coinbase.com/wrapped-assets/CBETH/conversion-rate)

### Response

#### 200 Success

A successful request responds with HTTP status code 200 (OK) and the JSON response body has the following form:

```
{
  "amount": "1.001374669367288075"
}

```

The `amount` field in the response body is the number of `ETH2` units that can be exchanged for 1 `cbETH`.

#### 429 Failure

This endpoint can be queried at most once a second. If queried more than once a second, the failed request responds with HTTP status code 429 ([Too Many Requests](https://docs.w3cub.com/http/status/429)) and the JSON response body has the following form:

```
{
  "message": "Public rate limit exceeded"
}

```

#### Path Parameters

#### Response