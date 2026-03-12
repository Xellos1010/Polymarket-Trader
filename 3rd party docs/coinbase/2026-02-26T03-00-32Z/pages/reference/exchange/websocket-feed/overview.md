# overview

The WebSocket feed is publicly available and provides real-time market data updates for orders and trades. Two endpoints are supported in both production and sandbox:

-   **Coinbase Market Data** is our traditional feed which is available without authentication.
-   **Coinbase Direct Market Data** has direct access to Coinbase Exchange servers and requires [Authentication](https://developer.chrome.com/exchange/websocket-feed/authentication).

## Protocol

The WebSocket feed uses a bidirectional protocol that encodes all messages as JSON objects. All messages have a `type` attribute that can be used to handle the message appropriately.

## Subscribe

To begin receiving feed messages, you must send a `subscribe` message to the server indicating which channels and products to receive. This message is mandatory—you are disconnected if no `subscribe` has been received within 5 seconds.

```
// Request
// Subscribe to ETH-USD and ETH-EUR with the level2, heartbeat and ticker channels,
// plus receive the ticker entries for ETH-BTC and ETH-USD
{
  "type": "subscribe",
  "product_ids": ["ETH-USD", "ETH-EUR"],
  "channels": [
    "level2",
    "heartbeat",
    {
      "name": "ticker",
      "product_ids": ["ETH-BTC", "ETH-USD"]
    }
  ]
}

```

You receive a `subscriptions` message as a response to an `subscribe` message.

### Unsubscribe

To unsubscribe from channel/product pairs, send an `unsubscribe` message. The structure is equivalent to `subscribe` messages.

```
// Request
{
  "type": "unsubscribe",
  "channels": ["heartbeat"]
}

```

You receive a `subscriptions` message as a response to an `unsubscribe` message.

### Specifying Product IDs

There are two ways to specify the product IDs to listen for within each channel:

-   You can define product IDs for an individual channel.
-   You can define product IDs at the root of the object—this adds them to all the channels you subscribe to.

```
// Request
{
  "type": "unsubscribe",
  "product_ids": ["ETH-USD", "ETH-EUR"],
  "channels": ["ticker"]
}

```

### Subscriptions Message

A `subscriptions` message is sent in response to both [subscribe](#subscribe) and [unsubscribe](#unsubscribe) messages. In response to a `subscribe` message, the `subscriptions` message lists all channels you are subscribed to. Subsequent subscribe messages add to the list of subscriptions. If you subscribed to a channel without being authenticated, you will remain in the unauthenticated channel.

```
// Response
{
  "type": "subscriptions",
  "channels": [
    {
      "name": "level2",
      "product_ids": ["ETH-USD", "ETH-EUR"]
    },
    {
      "name": "heartbeat",
      "product_ids": ["ETH-USD", "ETH-EUR"]
    },
    {
      "name": "ticker",
      "product_ids": ["ETH-USD", "ETH-EUR", "ETH-BTC"]
    }
  ]
}

```

## Websocket Compression Extension

Websocket compression, defined in RFC7692, compresses the payload of WebSocket messages which can increase total throughput and potentially reduce message delivery latency. The **permessage-deflate extension** can be enabled by adding the extension header. Currently, it is not possible to specify the compression level. From [RFC7692](https://datatracker.ietf.org/doc/html/rfc7692#section-7.1.3): The simplest “Sec-WebSocket-Extensions” header in a client (or server’s) opening handshake to offer (or accept) use of the “permessage-deflate” extension looks like this:

```
GET wss://ws-feed.exchange.coinbase.com
Sec-WebSocket-Extensions: permessage-deflate

```

## Sequence Numbers

Most feed messages contain a sequence number. Sequence numbers are increasing integer values for each product, with each new message being exactly one sequence number greater than the one before it. Sequence numbers that are *greater than one integer value* from the previous number indicate that a message has been dropped. Sequence numbers that are *less* than the previous number can be ignored or represent a message that has arrived out of order. In either situation you may need to perform logic to make sure your system is in the correct state.

## End-to-end Example

Below is an end-to-end example for Python that handles authentication and connection to the Exchange WebSocket. This code sample can be cloned at [Coinbase Samples](https://github.com/coinbase-samples/exchange-scripts-py/tree/main/websocket).

```

import asyncio, base64, hashlib, hmac, json, os, time, websockets
API_KEY = str(os.environ.get('API_KEY'))
PASSPHRASE = str(os.environ.get('PASSPHRASE'))
SECRET_KEY = str(os.environ.get('SECRET_KEY'))
URI = 'wss://ws-feed.exchange.coinbase.com'
SIGNATURE_PATH = '/users/self/verify'
channel = 'level2'
product_ids = 'ETH-USD'
async def generate_signature():
    timestamp = str(time.time())
    message = f'{timestamp}GET{SIGNATURE_PATH}'
    hmac_key = base64.b64decode(SECRET_KEY)
    signature = hmac.new(
        hmac_key,
        message.encode('utf-8'),
        digestmod=hashlib.sha256).digest()
    signature_b64 = base64.b64encode(signature).decode().rstrip('\n')
    return signature_b64, timestamp
async def websocket_listener():
    signature_b64, timestamp = await generate_signature()
    subscribe_message = json.dumps({
        'type': 'subscribe',
        'channels': [{'name': channel, 'product_ids': [product_ids]}],
        'signature': signature_b64,
        'key': API_KEY,
        'passphrase': PASSPHRASE,
        'timestamp': timestamp
    })
    while True:
        try:
            async with websockets.connect(URI, ping_interval=None) as websocket:
                await websocket.send(subscribe_message)
                while True:
                    response = await websocket.recv()
                    json_response = json.loads(response)
                    print(json_response)
        except (websockets.exceptions.ConnectionClosedError, websockets.exceptions.ConnectionClosedOK):
            print('Connection closed, retrying..')
            await asyncio.sleep(1)
if __name__ == '__main__':
    try:
        asyncio.run(websocket_listener())
    except KeyboardInterrupt:
        print("Exiting WebSocket..")

```