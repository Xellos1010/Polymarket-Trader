# rate limiting

The Coinbase Business API is rate limited to prevent abuse that would degrade our ability to maintain consistent API performance for all users. By default, each API key or app is rate limited at **10,000 requests per hour**. If your requests are being rate limited, HTTP response code `429` is returned with an `rate_limit_exceeded` error.

> Rate limiting error (429)

```
{
  "errors": [
    {
      "id": "rate_limit_exceeded",
      "message": "Too many requests"
    }
  ]
}

```