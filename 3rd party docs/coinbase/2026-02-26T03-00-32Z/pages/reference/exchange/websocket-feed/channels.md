# channels

## Heartbeat Channel

To receive heartbeat messages for specific products every second, subscribe to the `heartbeat` channel. Heartbeats include [sequence numbers](https://developer.chrome.com/exchange/websocket-feed/overview#sequence-numbers) and last trade IDs that can be used to verify that no messages were missed.

```
// Request
{
    "type": "subscribe",
    "channels": [
        {
            "name": "heartbeat",
            "product_ids": [
                "ETH-EUR"
            ]
        }
    ]
}

```

```
// Heartbeat message
{
  "type": "heartbeat",
  "sequence": 90,
  "last_trade_id": 20,
  "product_id": "BTC-USD",
  "time": "2014-11-07T08:19:28.464459Z"
}

```

## Status Channel

The `status` channel sends all products and currencies on a preset interval.

```
// Request
{
  "type": "subscribe",
  "channels": [{ "name": "status"}]
}

```

```
// Status Message
{
  "type": "status",
  "products": [
    {
      "id": "BTC-USD",
      "base_currency": "BTC",
      "quote_currency": "USD",
      "base_increment": "0.00000001",
      "quote_increment": "0.01",
      "display_name": "BTC-USD",
      "status": "online",
      "status_message": null,
      "min_market_funds": "10",
      "post_only": false,
      "limit_only": false,
      "cancel_only": false,
      "fx_stablecoin": false
    }
  ],
  "currencies": [
    {
      "id": "USD",
      "name": "United States Dollar",
      "display_name": "USD",
      "min_size": "0.01000000",
      "status": "online",
      "status_message": null,
      "max_precision": "0.01",
      "convertible_to": ["USDC"],
      "details": {},
      "default_network": "",
      "supported_networks": []
    },
    {
      "id": "USDC",
      "name": "USD Coin",
      "display_name": "USDC",
      "min_size": "0.00000100",
      "status": "online",
      "status_message": null,
      "max_precision": "0.000001",
      "convertible_to": ["USD"],
      "details": {},
      "default_network": "ethereum",
      "supported_networks": [
        {
          "id": "ethereum",
          "name": "Ethereum",
          "status": "online",
          "contract_address": "",
          "crypto_address_link": "https://etherscan.io/token/0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48?a={{address}}",
          "crypto_transaction_link": "https://etherscan.io/tx/0x{{txId}}",
          "min_withdrawal_amount": 0.001,
          "max_withdrawal_amount": 300000000,
          "network_confirmations": 14,
          "processing_time_seconds": 0,
          "destination_tag_regex": ""
        }
      ]
    },
    {
      "id": "BTC",
      "name": "Bitcoin",
      "display_name": "BTC",
      "min_size":" 0.00000001",
      "status": "online",
      "status_message": null,
      "max_precision": "0.00000001",
      "convertible_to": [],
      "details": {},
      "default_network": "bitcoin",
      "supported_networks": [
        {
          "id": "bitcoin",
          "name": "Bitcoin",
          "status": "online",
          "contract_address": "",
          "crypto_address_link": "https://live.blockcypher.com/btc/address/{{address}}",
          "crypto_transaction_link": "https://live.blockcypher.com/btc/tx/{{txId}}",
          "min_withdrawal_amount": 0.0001,
          "max_withdrawal_amount": 2400,
          "network_confirmations": 2,
          "processing_time_seconds": 0,
          "destination_tag_regex": ""
        }
      ]
    }
  ]
}

```

## Auction Channel

The `auction` channel sends information about the auction while the product is in auction mode. Auction messages provide the most recent indicative quote disseminated during the auction. Indicative quote messages are sent on an interval basis (about once a second) during the collection phase of an auction. The indicative quote includes information about the tentative price and size affiliated with the completion. The open price and size indicate the aggregate size of all the orders eligible for crossing, along with the price used for matching all the orders as the auction enters the opening state. The best bid and ask price and size fields indicate the anticipated BBO upon entering full trading or limit only after the matching has completed. Because indicative quotes are sent on an interval, values are not firm. The price may change in between two quote updates: (1) in between two normal quote update intervals, or (2) in between the last normal quote update interval and the final indicative quote that occurs when the book transitions from auction mode to full trading. See [Get Product Book](https://developer.chrome.com/api-reference/exchange-api/rest-api/products/get-product-book) in the API Reference for more details on the level 1 book and products in auction mode.

```
// Request
{
  "type": "subscribe",
  "channels": [{ "name": "auctionfeed", "product_ids": ["LTC-USD"] }]
}

```

```
// Auction Message
{
    "type": "auction",
    "product_id": "LTC-USD",
    "sequence": 3262786978,
    "auction_state": "collection",
    "best_bid_price": "333.98",
    "best_bid_size": "4.39088265",
    "best_ask_price": "333.99",
    "best_ask_size": "25.23542881",
    "open_price": "333.99",
    "open_size": "0.193",
    "can_open": "yes",
    "timestamp": "2015-11-14T20:46:03.511254Z"
}

```

## Matches Channel

If you are only interested in [match](#match) messages, you can subscribe to the matches channel. This is useful when you’re consuming the remaining feed using the [level2 channel](#level2-channel). After subscribing to the matches channel, the message `type` of the first message returned (and only the first message) is `last_match`, for example, `"type": "last_match",`

## RFQ Matches Channel

The subscription message for the Request For Quote or `rfq_matches` channel does not require the `product_ids` field; otherwise, it is the same as all other WebSocket feed channels.

-   If `product_ids` is not sent, or sent as an empty string "", or sent as “ALL”, the user receives `rfq_matches` for all products.
-   If `product_ids` is defined, the subscriber only receives `rfq_matches` for that product. The product specified must be a valid Coinbase product ID.

```
// Subscription Request
{
  "type": "subscriptions",
  "channels": [
    {
      "name": "rfq_matches",
      "product_ids": [
        "",
      ],
    },
  ]
}

```

```
// RFQ Request
{
  "type": "rfq_match",
  "maker_order_id": "ac928c66-ca53-498f-9c13-a110027a60e8",
  "taker_order_id": "132fb6ae-456b-4654-b4e0-d681ac05cea1",
  "time": "2014-11-07T08:19:27.028459Z",
  "trade_id": 30,
  "product_id": "BTC-USD",
  "size": "5.23512",
  "price": "400.23",
  "side": "sell"
}

```

## Ticker Channel

The `ticker` channel provides real-time price updates every time a match happens. It batches updates in case of cascading matches, greatly reducing bandwidth requirements.

```
// Request
{
    "type": "subscribe",
    "product_ids": [
        "ETH-USD",
        "BTC-USD"
    ],
    "channels": ["ticker"]
}

```

```
// Ticker message
{
  "type": "ticker",
  "sequence": 37475248783,
  "product_id": "ETH-USD",
  "price": "1285.22",
  "open_24h": "1310.79",
  "volume_24h": "245532.79269678",
  "low_24h": "1280.52",
  "high_24h": "1313.8",
  "volume_30d": "9788783.60117027",
  "best_bid": "1285.04",
  "best_bid_size": "0.46688654",
  "best_ask": "1285.27",
  "best_ask_size": "1.56637040",
  "side": "buy",
  "time": "2022-10-19T23:28:22.061769Z",
  "trade_id": 370843401,
  "last_size": "11.4396987"
}

```

## Ticker Batch Channel

The `ticker_batch` channel provides latest price updates **every 5000 milliseconds** (5 seconds) if there is a change. It has the same JSON message schema as the [ticker channel](#ticker-channel).

```
// Request
{
    "type": "subscribe",
    "product_ids": [
        "ETH-USD",
        "BTC-USD"
    ],
    "channels": ["ticker_batch"]
}

```

## Full Channel

The `full` channel provides real-time updates on orders and trades. These updates can be applied to a level3 order book snapshot to maintain an accurate and up-to-date copy of the exchange order book. To maintain an up-to-date level3 order book:

1.  Send a `subscribe` message for the product(s) of interest and the `full` channel.
2.  Queue any messages received over the websocket stream.
3.  Make a REST request for the order book snapshot from the REST feed.
4.  Playback queued messages, discarding sequence numbers before or equal to the snapshot sequence number.
5.  Apply playback messages to the snapshot as needed (see below).
6.  After playback is complete, apply real-time stream messages as they arrive.

The following messages are sent over the websocket stream in JSON format when subscribing to the full channel:

### Received

*A valid order has been received and is now active.* This message is emitted for every single valid order as soon as the matching engine receives it, whether it fills immediately or not. The `received` message does not indicate a resting order on the order book. The `received` message indicates that a new incoming order has been accepted by the matching engine for processing. Received orders may cause `match` message to follow if they are able to begin being filled (taker behavior). [Self-trade prevention](https://developer.chrome.com/exchange/concepts/matching-engine#self-trade-prevention) may also trigger `change` messages to follow if the order size needs to be adjusted. Orders that are not fully filled or that are canceled due to self-trade prevention, result in an `open` message and become resting orders on the order book. Market orders (indicated by the `order_type` field) may have an optional `funds` field which indicates how much quote currency is used to buy or sell. For example, a `funds` field of `100.00` for the `BTC-USD` product would indicate a purchase of up to `100.00 USD` worth of bitcoin.

Received message for limit order:

```
{
  "type": "received",
  "time": "2014-11-07T08:19:27.028459Z",
  "product_id": "BTC-USD",
  "sequence": 10,
  "order_id": "d50ec984-77a8-460a-b958-66f114b0de9b",
  "size": "1.34",
  "price": "502.1",
  "side": "buy",
  "order_type": "limit",
  "client_oid": "d50ec974-76a2-454b-66f135b1ea8c"
}

```

Received message for market order:

```
{
  "type": "received",
  "time": "2014-11-09T08:19:27.028459Z",
  "product_id": "BTC-USD",
  "sequence": 12,
  "order_id": "dddec984-77a8-460a-b958-66f114b0de9b",
  "funds": "3000.234",
  "side": "buy",
  "order_type": "market",
  "client_oid": "d50ec974-76a2-454b-66f135b1ea8c"
}

```

### Open

*The order is now open on the order book.* This message is only sent for orders that are not fully filled immediately. `remaining_size` indicates how much of the order is unfilled and going on the book.

```
{
  "type": "open",
  "time": "2014-11-07T08:19:27.028459Z",
  "product_id": "BTC-USD",
  "sequence": 10,
  "order_id": "d50ec984-77a8-460a-b958-66f114b0de9b",
  "price": "200.2",
  "remaining_size": "1.00",
  "side": "sell"
}

```

### Done

*The order is no longer on the order book.* This message is sent for all orders for which there was a received message and can result from an order being canceled or filled. There are no more messages for an `order_id` after a done message. `remaining_size` indicates how much of the order went unfilled; this is `0` for `filled` orders. `market` orders do not have a `remaining_size` or `price` field as they are never on the open order book at a given price.

```
{
  "type": "done",
  "time": "2014-11-07T08:19:27.028459Z",
  "product_id": "BTC-USD",
  "sequence": 10,
  "price": "200.2",
  "order_id": "d50ec984-77a8-460a-b958-66f114b0de9b",
  "reason": "filled", // or "canceled"
  "side": "sell",
  "remaining_size": "0"
}

```

#### Cancel Reason

Done messages with `reason=canceled` (that are authenticated and that originated with you the user) return the reason in the `cancel_reason` field: Supported cancel reasons are:

```
101:Time In Force
102:Self Trade Prevention
103:Admin
104:Price Bound Order Protection
105:Insufficient Funds
106:Insufficient Liquidity
107:Broker

```

### Match

*A trade occurred between two orders.* The aggressor or `taker` order is the one executing immediately after being received and the `maker` order is a resting order on the book. The `side` field indicates the maker order side. If the side is `sell` this indicates the maker was a sell order and the `match` is considered an up-tick. A `buy` side match is a down-tick.

```
{
  "type": "match",
  "trade_id": 10,
  "sequence": 50,
  "maker_order_id": "ac928c66-ca53-498f-9c13-a110027a60e8",
  "taker_order_id": "132fb6ae-456b-4654-b4e0-d681ac05cea1",
  "time": "2014-11-07T08:19:27.028459Z",
  "product_id": "BTC-USD",
  "size": "5.23512",
  "price": "400.23",
  "side": "sell"
}

```

If authenticated, and you were the taker, the message would also have the following fields:

```
{
  ...
  "taker_user_id": "5844eceecf7e803e259d0365",
  "user_id": "5844eceecf7e803e259d0365",
  "taker_profile_id": "765d1549-9660-4be2-97d4-fa2d65fa3352",
  "profile_id": "765d1549-9660-4be2-97d4-fa2d65fa3352",
  "taker_fee_rate": "0.005"
}

```

Similarly, if you were the maker, the message would have the following:

```
{
  ...
  "maker_user_id": "5f8a07f17b7a102330be40a3",
  "user_id": "5f8a07f17b7a102330be40a3",
  "maker_profile_id": "7aa6b75c-0ff1-11eb-adc1-0242ac120002",
  "profile_id": "7aa6b75c-0ff1-11eb-adc1-0242ac120002",
  "maker_fee_rate": "0.001"
}

```

### Change

*An order has changed.* A `change` message can be the result of either a [Self-trade Prevention (STP)](https://developer.chrome.com/exchange/concepts/matching-engine#self-trade-prevention) or a Modify Order Request:

-   A Self-trade Prevention adjusts the order size or available funds (and can only decrease).
-   A Modify Order Request adjusts the order size or price.

`change` messages are sent anytime an order changes in size or price. This includes:

-   Orders that are open (resting)
-   Orders that are received but not yet open.
-   Market orders with `funds` changed from a Self-trade Prevention control.

> If you are building from a level2 book, the `side` and `price` fields to indicate whether the change message is relevant. STP Change messages for limit orders always have a price specified. STP change messages for market orders have no price (`null`) and a decrease in order size.

Example of a change message from a Self-trade Prevention action:

> STP messages have a new `reason` field and continue to use the `price` field (not `new_price` and `old_price`).

```
{
  "type": "change",
  "reason":"STP",
  "time": "2014-11-07T08:19:27.028459Z",
  "sequence": 80,
  "order_id": "ac928c66-ca53-498f-9c13-a110027a60e8",
  "side": "sell",
  "product_id": "BTC-USD",
  "old_size": "12.234412",
  "new_size": "5.23512",
  "price": "400.23"
}

```

Example of a change message from a Modify Order Request:

> Modify Order messages add three new fields: `new_price`, `old_price`, `reason`.

```
{
  "type": "change",
  "reason":"modify_order",
  "time": "2022-06-06T22:55:43.433114Z",
  "sequence": 24753,
  "order_id": "c3f16063-77b1-408f-a743-88b7bc20cdcd",
  "side": "buy",
  "product_id": "ETH-USD",
  "old_size": "80",
  "new_size": "80",
  "old_price": "7",
  "new_price": "6"
}

```

### Activate

*An activate message is sent when a stop order is placed.* When the stop is triggered the order is placed and goes through the [order lifecycle](https://developer.chrome.com/exchange/concepts/matching-engine#order-lifecycle).

```
{
  "type": "activate",
  "product_id": "test-product",
  "timestamp": "1483736448.299000",
  "user_id": "12",
  "profile_id": "30000727-d308-cf50-7b1c-c06deb1934fc",
  "order_id": "7b52009b-64fd-0a2a-49e6-d8a939753077",
  "stop_type": "entry",
  "side": "buy",
  "stop_price": "80",
  "size": "2",
  "funds": "50",
  "private": true
}

```

## User Channel

The `user` channel is a version of the [full channel](#full-channel) and only contains messages that include the authenticated user. Consequently, you need to be [authenticated](https://developer.chrome.com/exchange/websocket-feed/authentication#authentication) to receive any messages.

## Level2 Channel

The `level2` channel guarantees delivery of all updates and is the easiest way to keep a snapshot of the order book. This channel also reduces the overhead required when consuming the [full channel](#full-channel).

```
// Request
{
    "type": "subscribe",
    "channels": ["level2"],
    "product_ids": [
        "ETH-USD",
        "BTC-USD"
    ]
}

```

The `level2` channel sends a message with the type `snapshot` and the corresponding `product_id`. The properties `bids` and `asks` are arrays of `[price, size]` tuples and represent the entire order book.

```
{
  "type": "snapshot",
  "product_id": "BTC-USD",
  "bids": [["10101.10", "0.45054140"]],
  "asks": [["10102.55", "0.57753524"]]
}

```

Subsequent updates have the type `l2update`. The `changes` property of `l2update`s is an array with `[side, price, size]` tuples. The `time` property of `l2update` is the time of the event as recorded by our trading engine.

##### Single `changes` Array

```
{
  "type": "l2update",
  "product_id": "BTC-USD",
  "time": "2019-08-14T20:42:27.265Z",
  "changes": [
    [
      "buy",
      "10101.80000000",
      "0.162567"
    ]
  ]
}

```

##### Multiple `changes` Arrays

```
{
  "type": "l2update",
  "product_id": "BTC-USD",
  "changes": [
    [
      "buy",
      "22356.270000",
      "0.00000000"
    ],
    [
      "buy",
      "22356.300000",
      "1.00000000"
    ]
  ],
  "time": "2022-08-04T15:25:05.010758Z"
}

```

## Level2 Batch Channel

The `level2_batch` channel sends batches of `level2` messages **every 50 milliseconds** (0.05 seconds). It has the same JSON message schema as the [`level2` channel](#level2-channel). The time field correlates to the most recent message in the batch.

```
// Request
{
    "type": "subscribe",
    "product_ids": [
        "ETH-USD",
        "BTC-USD"
    ],
    "channels": ["level2_batch"]
}

```

## Level3 Channel

The `level3` channel is a compact version of the Full channel. It conveys all of the same data in a compact message structure that requires less bandwidth with potentially more efficient client side parsing.

```
// Subscribe request
{
    "type": "subscribe",
    "channels": ["level3"],
    "product_ids": [
        "ETH-USD",
        "BTC-USD"
    ]
}

```

### L3 Schema

On subscribe, the first response returns a level3 schema with the structure for each message type. The schema is not repeated.

Expand to view the full level3 schema

```
{
  "type": "level3",
  "schema": {
    "change": [
      "type",
      "product_id",
      "sequence",
      "order_id",
      "price",
      "size",
      "time"
    ],
    "done": [
      "type",
      "product_id",
      "sequence",
      "order_id",
      "time"
    ],
    "match": [
      "type",
      "product_id",
      "sequence",
      "maker_order_id",
      "taker_order_id",
      "price",
      "size",
      "time"
    ],
    "noop": [
      "type",
      "product_id",
      "sequence",
      "time"
    ],
    "open": [
      "type",
      "product_id",
      "sequence",
      "order_id",
      "side",
      "price",
      "size",
      "time"
    ]
  }
}

```

Subsequent messages for each type pack the data into an array with a structure as defined in the initial response, for example:

```
[
  "open",
  "BTC-USD",
  "57560479456",
  "12aca6e0-7400-418a-9e59-c0020a3bf8cc",
  "buy",
  "27268.09",
  "0.02",
  "2023-03-28T23:24:03.185394Z"
]

```

## Balance Channel

The `balance` channel tracks account balance updates, which is useful for checking the holds and available balance on your account. It does *not* track every update. Authentication is required.

#### Fields

A response from the channel includes the following fields:

```
{
  "type": "balance",
  "account_id": "d50ec984-77a8-460a-b958-66f114b0de9b",
  "currency": "USD",
  "holds": "1000.23",                      // funds locked in account
  "available": "102030.99",                // balance available for trading
  "updated": "2023-10-10T20:42:27.265Z",   // when last balance change is observed
  "timestamp": "2023-10-10T20:42:29.265Z"  // when message is sent from websocket
}

```

#### Subscribe

Clients can subscribe to this channel using the following subscribe messages:

##### Example 1

```
{
  "type": "subscribe",
  "channels": [
    {
      "name": "balance",
      "account_ids": [
        "d50ec984-77a8-460a-b958-66f114b0de9b",
        "d50ec984-77a8-460a-b958-66f114b0de9a"
      ]
    }
  ]
}

```

##### Example 2

```
{
  "type": "subscribe",
  "channels": [
    "balance"
  ],
  "account_ids": [
    "d50ec984-77a8-460a-b958-66f114b0de9b",
    "d50ec984-77a8-460a-b958-66f114b0de9a"
  ]
}

```