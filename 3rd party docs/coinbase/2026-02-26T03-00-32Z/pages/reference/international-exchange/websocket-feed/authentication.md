# authentication

You must authenticate when subscribing to the WebSocket Feed the very first time. To authenticate, include the authentication fields listed below on the subscribe message.

```
{
  "type": "SUBSCRIBE",
  "product_ids": ["BTC-PERP"],
  "channels": ["LEVEL2"],
  "time": "1683730727",
  "key": "glK4uG8QRmh3aqnJ",
  "passphrase": "passphrase",
  "signature": "1BM6nwNBLHAkLLs81qcKEKAAPoYIzxTuDIX9DpE0/EM="
}

```

## Authentication Fields

Field

Description

`key`

API key used to create the signature

`time`

Timestamp, in epoch seconds, when the request is made. Must be within 30 seconds of making the request.

`passphrase`

Passphrase affiliated with the API Key

`signature`

HMAC SHA-256 signature with the API Key secret on the concatenated string `TIMESTAMP + KEY + “CBINTLMD” + PASSPHRASE`. Example: `1744594506glK4uG8QRmh3aqnJCBINTLMDpassphrase`