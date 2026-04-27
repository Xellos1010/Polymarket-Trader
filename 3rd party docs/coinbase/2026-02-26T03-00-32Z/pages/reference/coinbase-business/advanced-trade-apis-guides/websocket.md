# websocket

## 1\. Introduction

This guide provides a comprehensive overview of setting up Coinbase Advanced Trade WebSockets, including essential steps for authentication and managing subscriptions. Coinbase’s Advanced Trade WebSocket API enables real-time access to market data and user-specific order information, empowering developers to build robust trading applications and monitoring tools. In this first part, you’ll learn how to establish WebSocket connections, authenticate using JWT tokens, and efficiently subscribe to key channels, such as market data and user-specific channels. With this foundational knowledge, you’ll be prepared to integrate the WebSocket API seamlessly into your application.

### Overview of WebSocket Functionality

The Coinbase Developer Platform’s Advanced Trade product provides two WebSocket endpoints:

-   **Market Data Endpoint**: `wss://advanced-trade-ws.coinbase.com` This public WebSocket feed delivers real-time updates on market orders and trades for various cryptocurrency products.
-   **User Order Data Endpoint**: `wss://advanced-trade-ws-user.coinbase.com` This authenticated WebSocket feed provides real-time updates on the user’s orders, including order status and updates on active trades.

Both endpoints provide live data streams using WebSocket connections, enabling developers to receive real-time updates on trading activity, order books, and market movements. This guide will cover how to correctly establish these WebSocket connections, manage subscriptions, and handle potential errors during integration.

## 2\. Setting Up WebSocket Connections

To integrate with the Coinbase Advanced Trade WebSockets, developers need to establish a WebSocket connection with either the Market Data or User Order Data endpoints. This section will guide you through the process of setting up these WebSocket connections and ensuring they remain active.

### WebSocket Endpoints

**Market Data Endpoint**: `wss://advanced-trade-ws.coinbase.com` This endpoint provides real-time market data, including updates on orders, trades, and price changes for various cryptocurrency pairs. Authentication is not required for most channels on this endpoint. **User Order Data Endpoint**: `wss://advanced-trade-ws-user.coinbase.com` This endpoint provides updates related to a user’s orders, including order status, fills, and real-time changes. It requires authentication using a JWT (JSON Web Token).

### Basic Connection Setup

After establishing a WebSocket connection, the server expects a subscription message to be sent within 5 seconds; otherwise, the connection will be terminated. This subscription message tells the WebSocket server which channels and products the client wants to receive data for. Developers can subscribe to multiple channels, but each subscription must be sent in a unique message.

### Example: Connecting Without Authentication (Market Data Endpoint)

For public data, you can set up a basic WebSocket connection without authentication. Here is an example in Python that connects to the market data WebSocket and subscribes to the `ticker` channel for the BTC-USD product. First, let’s install the necessary dependency:: `pip install websocket`

```
import websocket
import json
# Market Data WebSocket URL
WS_URL = "wss://advanced-trade-ws.coinbase.com"
def on_open(ws):
    # Subscribe to the ticker channel for BTC-USD
    subscribe_message = {
        "type": "subscribe",
        "product_ids": ["BTC-USD"],
        "channel": "ticker"
    }
    ws.send(json.dumps(subscribe_message))
    print("Subscribed to BTC-USD ticker channel")
def on_message(ws, message):
    data = json.loads(message)
    print(f"Received message: {data}")
def on_error(ws, error):
    print(f"Error: {error}")
def on_close(ws):
    print("Connection closed")
# Create the WebSocket connection
ws = websocket.WebSocketApp(
    WS_URL,
    on_open=on_open,
    on_message=on_message,
    on_error=on_error,
    on_close=on_close
)
ws.run_forever()

```

In this example, once the connection is established, the client sends a subscription message to the `ticker` channel for the BTC-USD product. The WebSocket then begins streaming real-time price updates for that product.

### Example: Connecting With Authentication (User Order Data Endpoint)

For user-specific data, such as order updates, you must authenticate using a JWT. Below is an example in Python that connects to the User Order Data WebSocket and subscribes to the `user` channel. First, let’s install the necessary dependencies:

```
pip install websocket "pyjwt[crypto]"
import websocket
import json
import jwt  # PyJWT library
import time
import uuid
# User Order Data WebSocket URL
WS_USER_URL = "wss://advanced-trade-ws-user.coinbase.com"
# Example JWT generation (Replace with your actual API_KEY and SIGNING_KEY)
API_KEY = "organizations/{org_id}/apiKeys/{key_id}"
SIGNING_KEY = "-----BEGIN EC PRIVATE KEY-----\nYOUR_PRIVATE_KEY_HERE\n-----END EC PRIVATE KEY-----\n"
def generate_jwt():
    current_time = int(time.time())
    payload = {
        "iss": "cdp",
        "nbf": current_time,
        "exp": current_time + 120,  # JWT valid for 120 seconds
        "sub": API_KEY,
    }
    headers = {
        "kid": API_KEY,
        "nonce": uuid.uuid4().hex
    }
    return jwt.encode(payload, SIGNING_KEY, algorithm="ES256", headers=headers)
def on_open(ws):
    # Generate JWT
    token = generate_jwt()
    # Subscribe to the user channel for BTC-USD orders
    subscribe_message = {
        "type": "subscribe",
        "channel": "user",
        "product_ids": ["BTC-USD"],
        "jwt": token
    }
    ws.send(json.dumps(subscribe_message))
    print("Subscribed to user channel for BTC-USD orders")
def on_message(ws, message):
    data = json.loads(message)
    print(f"Received message: {data}")
def on_error(ws, error):
    print(f"Error: {error}")
def on_close(ws):
    print("Connection closed")
# Create the WebSocket connection
ws = websocket.WebSocketApp(
    WS_USER_URL,
    on_open=on_open,
    on_message=on_message,
    on_error=on_error,
    on_close=on_close
)
ws.run_forever()

```

In this authenticated example, we generate a JWT using the PyJWT library and include it in the subscription message for the user channel. This allows the client to receive real-time updates related to the user’s orders for the BTC-USD product.

WebSocket connections that interact with user-specific data on the Coinbase Developer Platform’s Advanced Trade product require authentication via JSON Web Tokens (JWTs). This section will guide developers through generating JWTs, subscribing to channels, and managing multiple subscriptions effectively.

### Generating and Using JWT Tokens

To subscribe to user-specific channels, such as the `user` or `futures_balance_summary` channels, you must include a valid JWT in the subscription message. The JWT is generated using your API key and signing key.

### Steps to Generate a JWT Token:

-   Obtain your API key and signing key from the Coinbase Developer Platform.
-   Use the JWT to authenticate when subscribing to user-related data channels.
-   JWT tokens expire after 2 minutes, so they must be refreshed regularly.

### Example: Generating a JWT in Python

First, ensure you have the required dependencies installed: `pip install websocket-client PyJWT cryptography` Then, generate the JWT for authentication:

```
import jwt  # PyJWT library
import time
import uuid
# Example API and Signing Keys (replace with actual values)
API_KEY = "organizations/{org_id}/apiKeys/{key_id}"
SIGNING_KEY = "-----BEGIN EC PRIVATE KEY-----\nYOUR_PRIVATE_KEY_HERE\n-----END EC PRIVATE KEY-----\n"
def generate_jwt():
    current_time = int(time.time())
    payload = {
        "iss": "cdp",
        "nbf": current_time,
        "exp": current_time + 120,  # valid for 120 seconds
        "sub": API_KEY,
    }
    headers = {
        "kid": API_KEY,
        "nonce": uuid.uuid4().hex
    }
    token = jwt.encode(payload, SIGNING_KEY, algorithm="ES256", headers=headers)
    return token
# Example JWT
token = generate_jwt()
print(f"Generated JWT: {token}")

```

This JWT must be included in all subscription messages for user-specific channels.

### Subscribing to Channels

To receive data from WebSocket channels, you need to send a subscription message after establishing the WebSocket connection. Each channel subscription must specify the product IDs (e.g., BTC-USD, ETH-USD) and, if required, a JWT for authentication.

### Example Subscription Message (Authenticated User Channel)

Here’s how to subscribe to the `user` channel for receiving updates on your orders:

```
subscribe_message = {
    "type": "subscribe",
    "channel": "user",
    "product_ids": ["BTC-USD"],
    "jwt": generate_jwt()
}

```

### Example Subscription Message (Market Data Channel)

For public market data, no JWT is required:

```
subscribe_message = {
    "type": "subscribe",
    "product_ids": ["BTC-USD", "ETH-USD"],
    "channel": "ticker"
}

```

### Channel Overview

Coinbase Advanced Trade WebSockets provide access to multiple channels, each serving different purposes. Below is an overview of the available channels:

Channel

Description

Authentication Required

ticker

Real-time price updates every time a match happens

No

ticker\_batch

Real-time price updates every 5000 milliseconds

No

market\_trades

Real-time updates every time a market trade happens

No

status

Sends all products and currencies on a preset interval

No

level2

All updates and easiest way to keep order book snapshot

No

candles

Price updates aggregated into time intervals

No

user

Only sends messages that include the authenticated user

Yes

heartbeats

Real-time server pings to keep all connections open

No

futures\_balance\_summary

Real-time updates every time a user’s futures balance changes

Yes

Each subscription can handle only one channel at a time, so if you want to receive data from multiple channels, send a separate subscription message for each.

### Best Practices for Subscriptions

1.  **Load Balancing Across WebSocket Connections** When subscribing to multiple channels or products, it is recommended to spread the load across different WebSocket connections. For example, instead of subscribing to multiple high-volume products (like BTC-USD and ETH-USD) on the same connection, open separate WebSocket connections for each. This reduces the risk of dropped messages and helps distribute inbound traffic more efficiently.
2.  **Using the Heartbeats Channel** The `heartbeats` channel sends periodic heartbeat messages to ensure the WebSocket connection remains active, especially when there are no frequent updates in other channels (e.g., illiquid markets). It is a best practice to subscribe to the `heartbeats` channel alongside other channels to prevent connections from closing due to inactivity.

### Example Heartbeats Subscription:

```
subscribe_message = {
    "type": "subscribe",
    "channel": "heartbeats"
}

```

Including the heartbeats subscription ensures that the WebSocket connection stays open, even when there is a lack of updates on other subscribed channels.

## 4\. WebSocket Channels Overview

Coinbase’s Advanced Trade WebSockets offer various channels that provide real-time market data and user-specific information. This section gives an overview of the available channels, indicates which channels require authentication, and provides sample code to subscribe and unsubscribe from these channels.

### Which Channels Require Authentication?

-   Public Channels (No Authentication Required): These channels are used to receive market data and do not require authentication. They include:
    -   level2
    -   ticker
    -   ticker\_batch
    -   candles
    -   heartbeats
    -   market\_trades
    -   status
-   Private Channels (JWT Authentication Required): These channels provide user-specific data and require the inclusion of a valid JWT token in the subscription message. They include:
    -   user
    -   futures\_balance\_summary

### Sample Code for Subscriptions

Below are examples of how to subscribe to and unsubscribe from various channels using Python. These examples demonstrate both public and private channels.

### Subscribing to a Public Channel (e.g., Ticker Channel)

Public channels do not require authentication, so you can simply send a subscription message after establishing the WebSocket connection.

```
import websocket
import json
def on_open(ws):
    # Subscribe to the ticker channel for BTC-USD
    subscribe_message = {
        "type": "subscribe",
        "product_ids": ["BTC-USD"],
        "channel": "ticker"
    }
    ws.send(json.dumps(subscribe_message))
    print("Subscribed to BTC-USD ticker channel")
# Set up WebSocket connection
ws = websocket.WebSocketApp("wss://advanced-trade-ws.coinbase.com", on_open=on_open)
ws.run_forever()

```

### Subscribing to a Private Channel (e.g., User Channel)

Private channels require a JWT for authentication. Use the following code to subscribe to the `user` channel:

```
import websocket
import json
def on_open(ws):
    jwt_token = generate_jwt()  # Replace with your JWT generation logic
    # Subscribe to the user channel
    subscribe_message = {
        "type": "subscribe",
        "channel": "user",
        "product_ids": ["BTC-USD"],
        "jwt": jwt_token
    }
    ws.send(json.dumps(subscribe_message))
    print("Subscribed to user channel for BTC-USD")
# Set up WebSocket connection
ws = websocket.WebSocketApp("wss://advanced-trade-ws-user.coinbase.com", on_open=on_open)
ws.run_forever()

```

### Unsubscribing from a Channel

To unsubscribe from a channel, send a message similar to the subscription message but with “type”: “unsubscribe”. You can unsubscribe from either public or private channels using this format. Example: Unsubscribing from the Ticker Channel

```
def unsubscribe(ws):
    unsubscribe_message = {
        "type": "unsubscribe",
        "product_ids": ["BTC-USD"],
        "channel": "ticker"
    }
    ws.send(json.dumps(unsubscribe_message))
    print("Unsubscribed from BTC-USD ticker channel")

```

### Example in JavaScript for Subscribing to a Channel

If developers are using JavaScript, they can use the ws library in a Node.js environment to subscribe to a WebSocket channel. Here’s how to subscribe to the ticker channel:

```
const WebSocket = require('ws');
const ws = new WebSocket('wss://advanced-trade-ws.coinbase.com');
ws.on('open', function open() {
  const subscribeMessage = JSON.stringify({
    type: "subscribe",
    product_ids: ["BTC-USD"],
    channel: "ticker"
  });
  
  ws.send(subscribeMessage);
  console.log("Subscribed to BTC-USD ticker channel");
});
ws.on('message', function incoming(data) {
  console.log(`Received: ${data}`);
});

```

## 5\. Conclusion

Successfully integrating with Coinbase’s Advanced Trade WebSocket API begins with a solid understanding of connection setup, authentication, and subscription management. Below is a summary of the essential best practices covered in this guide:

1.  **Ensuring Stable WebSocket Connections**
    -   Send a subscription message within 5 seconds of establishing a connection to prevent disconnection.
    -   Include the heartbeats channel in your subscriptions to keep connections alive during periods of low activity.
2.  **Authenticating with JWTs for Secure Data Access**
    -   Use JWT tokens for authenticated channels like the user channel.
    -   Refresh JWT tokens every 2 minutes to maintain uninterrupted access to private data channels.
3.  **Managing Subscriptions Across Channels**
    -   Use a unique subscription message for each channel and product combination.
    -   Spread subscriptions across multiple WebSocket connections to optimize data flow and maintain stability, especially when handling multiple high-volume channels.

By following these practices, you can build a robust, high-performance system capable of handling the demands of real-time data and trading insights. With this in-depth understanding of WebSocket optimization and advanced features, your application will be well-equipped to provide a seamless experience for end-users in Coinbase’s Advanced Trade environment.