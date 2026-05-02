# sdk websocket

This quickstart explains how to set up and subscribe to WebSocket channels with the **Advanced API Python WebSocket Client**. This WebSocket Client is a Python package that makes it easy to interact with the [WebSocket API](https://developer.chrome.com/coinbase-app/advanced-trade-apis/websocket/websocket-overview).

## Introduction

See the SDK [README](https://github.com/coinbase/coinbase-advanced-py/blob/master/README.md) for detailed instructions, plus the full suite of SDK functions.

## Prerequisites

### Creating API Keys

To you use the SDK, you must first create your own [API key](https://developer.chrome.com/coinbase-app/authentication-authorization/api-key-authentication) on the Coinbase Developer Platform (CDP).

### Installing the SDK

To install the Coinbase Advanced API Python SDK, run the following command in a terminal:

```
pip3 install coinbase-advanced-py

```

## Setting up your Client

Create a Python project with the following code we have set up for you. Replace the `api_key` and `api_secret` with your own CDP API Key and Secret. You must specify an `on_message` function that is called when a message is received from the WebSocket API. This function takes in a single argument, which is the raw message received from the WebSocket API. In the following example, we simply print the message to the console.

```
from coinbase.websocket import WSClient
api_key = "organizations/{org_id}/apiKeys/{key_id}"
api_secret = "-----BEGIN EC PRIVATE KEY-----\nYOUR PRIVATE KEY\n-----END EC PRIVATE KEY-----\n"
def on_message(msg):
    print(msg)
ws_client = WSClient(api_key=api_key, api_secret=api_secret, on_message=on_message, verbose=True)

```

You are now ready to start subscribing to channels!

### WebSocket User API Client

We offer a WebSocket User API client that allows you to connect to the Coinbase Advanced Trade WebSocket [heartbeats channel](https://developer.chrome.com/coinbase-app/advanced-trade-apis/websocket/websocket-channels#heartbeats-channel), [user channel](https://developer.chrome.com/coinbase-app/advanced-trade-apis/websocket/websocket-channels#user-channel) and [futures\_balance\_summary channel](https://developer.chrome.com/coinbase-app/advanced-trade-apis/websocket/websocket-channels#futures-balance-summary-channel). In your code, import the WSUserClient class instead of WSClient.

```
from coinbase.websocket import WSUserClient
api_key = "organizations/{org_id}/apiKeys/{key_id}"
api_secret = "-----BEGIN EC PRIVATE KEY-----\nYOUR PRIVATE KEY\n-----END EC PRIVATE KEY-----\n"
def on_message(msg):
    print(msg)
client = WSUserClient(api_key=api_key, api_secret=api_secret, on_message=on_message, verbose=True)

```

## Subscribing to your First Channels

Let’s start by opening a connection to the WebSocket API. Add the following call to `open` to your code, using the same client you just created.

With an open connection, you can now subscribe to channels.

-   The [Heartbeats](https://developer.chrome.com/coinbase-app/advanced-trade-apis/websocket/websocket-channels#heartbeats-channel) channel receives heartbeats messages for specific products every second, which is used to keep the connection alive.
-   The [Ticker](https://developer.chrome.com/coinbase-app/advanced-trade-apis/websocket/websocket-channels#ticker-channel) channel provides real-time price updates every time a match happens for a given product.

Using your same client, send the following message to subscribe to the `heartbeats` and `ticker` channels for the `BTC-USD` product. The received message is printed to the console.

```
ws_client.subscribe(["BTC-USD"], ["heartbeats", "ticker"])

```

## Running the Client for 10 Seconds

The above code only runs once and then exits. Use the `sleep_with_exception_check` method to run the client for a specified number of seconds. This method checks for exceptions every second, and exits if an exception is raised. Run the following code to keep the client open for 10 seconds. It prints all of the messages it receives, then closes the connection:

```
def on_message(msg):
    print(msg)
ws_client = WSClient(api_key=api_key, api_secret=api_secret, on_message=on_message, verbose=True)
ws_client.open()
ws_client.subscribe(["BTC-USD"], ["heartbeats", "ticker"])
ws_client.sleep_with_exception_check(10)
ws_client.close()

```

## Running the Client Indefinitely

To keep the client open indefinitely, use the `run_forever_with_exception_check` method. This method runs the client forever, and exits if an exception is raised.

```
def on_message(msg):
    print(msg)
ws_client = WSClient(api_key=api_key, api_secret=api_secret, on_message=on_message, verbose=True)
ws_client.open()
ws_client.subscribe(["BTC-USD"], ["heartbeats", "ticker"])
ws_client.run_forever_with_exception_check()

```

## Listening for Order Updates

Now let’s get a bit more… *advanced!* In this final section, we integrate both the REST and WebSocket APIs to:

1.  Place a limit-buy order 5% below said price (in the [REST SDK tutorial](https://developer.chrome.com/coinbase-app/advanced-trade-apis/guides/sdk-rest-api)).
2.  Subscribe to the [user channel](https://developer.chrome.com/coinbase-app/advanced-trade-apis/websocket/websocket-channels#user-channel) for updates on a Limit order.
3.  Print when the order is filled.

### Placing an Order 5% below Price

This section assumes that you used the REST SDK Client to [place a limit-buy order](https://developer.chrome.com/coinbase-app/advanced-trade-apis/guides/sdk-rest-api#placing-a-limit-buy-order) 5% below the current price of a product.

### Subscribing to the User Channel

Now let’s integrate the WebSocket SDK! Let’s call the [User](https://developer.chrome.com/coinbase-app/advanced-trade-apis/websocket/websocket-channels#user-channel) channel to receive updates on the order. This channel sends updates on all of a user’s open orders. First, let’s define the `on_message` function. This function:

-   Checks for all messages from the `user` channel’
-   Checks if the message is an update on our order.
-   Sets the `order_filled` variable to `True` if the order is filled.

```
def on_message(msg):
    global order_filled
    global limit_order_id
    message_data = json.loads(msg)
    if 'channel' in message_data and message_data['channel'] == 'user':
        orders = message_data['events'][0]['orders']
            for order in orders:
                order_id = order['order_id']
                if order_id == limit_order_id and order['status'] == 'FILLED':
                    order_filled = True

```

Now, let’s subscribe to the `user` and `heartbeats` channels and run the client in a `while` loop to wait for the order to be filled before closing the connection.

```
limit_order_id = ""
order_filled = False
ws_client = WSClient(api_key=api_key, api_secret=api_secret, on_message=on_message, verbose=True)
ws_client.open()
ws_client.subscribe(["BTC-USD"], ["heartbeats", "user"])
while not order_filled:
    ws_client.sleep_with_exception_check(1)
print(f"order {limit_order_id} filled!")
ws_client.close()

```

Lastly, let’s put it all together! We will integrate the REST SDK code from the [Placing a Limit-buy Order](https://developer.chrome.com/coinbase-app/advanced-trade-apis/guides/sdk-rest-api#placing-a-limit-buy-order) exercise with the WebSocket SDK code from the previous section.

```
from coinbase.rest import RESTClient
from coinbase.websocket import WSClient
import json
import math
api_key = "organizations/{org_id}/apiKeys/{key_id}"
api_secret = "-----BEGIN EC PRIVATE KEY-----\nYOUR PRIVATE KEY\n-----END EC PRIVATE KEY-----\n"
limit_order_id = ""
order_filled = False
def on_message(msg):
    global order_filled
    global limit_order_id
    message_data = json.loads(msg)
    if 'channel' in message_data and message_data['channel'] == 'user':
        orders = message_data['events'][0]['orders']
        for order in orders:
            order_id = order['order_id']
            if order_id == limit_order_id and order['status'] == 'FILLED':
                order_filled = True
# initialize REST and WebSocket clients
rest_client = RESTClient(api_key=api_key, api_secret=api_secret, verbose=True)
ws_client = WSClient(api_key=api_key, api_secret=api_secret, on_message=on_message, verbose=True)
# open WebSocket connection and subscribe to channels
ws_client.open()
ws_client.subscribe(["BTC-USD"], ["heartbeats", "user"])
# get current price of BTC-USD and place limit-buy order 5% below
product = rest_client.get_product("BTC-USD")
btc_usd_price = float(product["price"])
adjusted_btc_usd_price = str(math.floor(btc_usd_price - (btc_usd_price * 0.05)))
limit_order = rest_client.limit_order_gtc_buy(
    client_order_id="00000003",
    product_id="BTC-USD",
    base_size="0.0002",
    limit_price=adjusted_btc_usd_price
)
limit_order_id = limit_order["order_id"]
# wait for order to fill
while not order_filled:
    ws_client.sleep_with_exception_check(1)
print(f"order {limit_order_id} filled!")
ws_client.close()

```

And we are done! You can now use the WebSocket SDK to subscribe to channels and receive real-time updates from the Coinbase Advanced API.