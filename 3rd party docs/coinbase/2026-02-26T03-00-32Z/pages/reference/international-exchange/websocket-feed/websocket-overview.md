# websocket overview

The WebSocket feed is publicly available and provides real-time market data updates for orders and trades.

## Endpoints

-   **Production**: `wss://ws-md.international.coinbase.com`
-   **Sandbox**: `wss://ws-md.n5e2.coinbase.com`

## Protocol

The WebSocket feed uses a bidirectional protocol that encodes all messages as JSON objects. All messages have a `type` and `channel` attribute that can be used to handle the message appropriately.

## Subscribe

To begin receiving feed messages, you must send a `SUBSCRIBE` message to the server indicating which channels and products to receive. See [Authentication](https://developer.chrome.com/international-exchange/websocket-feed/authentication) for more details about the API key fields.

```
{
  "type": "SUBSCRIBE",
  "product_ids": ["BTC-PERP", "ETH-PERP"],
  "channels": ["MATCH", "INSTRUMENTS"],
  "time": "1685465606",
  "key": "glK4uG8QRmh3aqnJ",
  "passphrase": "passphrase",
  "signature": "1BM6nwNBLHAkLLs81qcKEKAAPoYIzxTuDIX9DpE0/EM="
}

```

A subscription confirmation with the current subscription information is sent as response.

```
{
  "channels": [
    {
      "name": "MATCH",
      "product_ids": ["BTC-PERP", "ETH-PERP"]
    },
    {
      "name": "INSTRUMENTS",
      "product_ids": ["BTC-PERP", "ETH-PERP"]
    }
  ],
  "authenticated": true,
  "channel": "SUBSCRIPTIONS",
  "type": "SNAPSHOT",
  "time": "2023-05-30T16:53:46.847Z"
}

```

## Unsubscribe

To unsubscribe from channel/product pairs, send an `UNSUBSCRIBE` message. The structure is equivalent to `SUBSCRIBE` messages.

```
{
  "type": "UNSUBSCRIBE",
  "product_ids": ["BTC-PERP"],
  "channels": ["MATCH"]
}

```

## Sequence Numbers

Most feed messages contain a sequence number. Sequence numbers are increasing integer values for the entire session, with each new message being exactly one sequence number greater than the one before it.

## Message Types

There are two message types: `SNAPSHOT` and `UPDATE`.

-   When you subscribe to a channel, a `SNAPSHOT` message is sent once on the subscription channel. This is the snapshot of the current state of the channel.
-   After you subscribe, all subsequent messages are of type `UPDATE`.