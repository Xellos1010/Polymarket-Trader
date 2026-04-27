# rate limits overview

Coinbase International Exchange enforces API rate limits at the individual API key level and at the account level.

## Summary

#### General Maximums

-   Maximum trading API Keys per account: 30
-   Maximum non-trading API Keys per account: 30
-   Maximum Drop Copy Sessions per account: 5
-   Maximum messages per second per account: 4000

#### API Limits

-   FIX Connections per API key: 1
-   FIX messages per second per API key: 800
-   REST requests per second per API Key: 40

## FIX API Rate Limits

-   Connections per API key: 1
-   Messages per second per API key: 800 (session disconnects at 1000)

## REST API Rate Limits

-   Requests per second per API Key: 40

### REST Errors

> HTTP Code: 429  
> Message: `Rate limit exceeded - Server allows 100 requests per second`

## WebSocket Connection Limits​

-   Maximum connection attempts every 30 seconds: 10